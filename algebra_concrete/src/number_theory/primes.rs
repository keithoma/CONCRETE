//! fn is_prime(n: u64) -> bool;
//! fn primes_up_to(n: u64) -> array;

//! implementations to find out if it's prime

//! TrialDivision

//! implementations for primes_up_to_n

//! the sieve of this greek guy

//! (each implementation of is_prime gives method for primes up_to)


//! also include the version we just look up a hash table
//! fn create table
//! fn look up table

//! functions for proof of concept
//! fn array_needed(type: integer_type) -> integer_type;




trait Primality {
    /// Returns `true` if `n` is prime and `false` otherwise.
    fn is_prime(n: u64) -> bool;
}

trait PrimeGeneration {
    /// Returns a vector with prime numbers up to and including `n`.
    fn primes_up_to(n: u64) -> Vec<u64>;
}

trait PrimalityFromGeneration: PrimeGeneration {
    /// Returns `true` if `n` is prime and `false` otherwise. Uses
    /// `primes_up_to(n)` from `PrimeGeneration` trait.
    fn is_prime(n: u64) -> bool {
        n > 1 && Self::primes_up_to(n).last() == Some(&n)
    }
}

trait PrimeGenerationFromPrimality: Primality {
    /// Returns a vector with prime numbers up to and including `n`. Uses
    /// `is_prime(n)` from `Primality` trait.
    fn primes_up_to(n: u64) -> Vec<u64> {
        (2..=n)
            .filter(|&x| Self::is_prime(x))
            .collect::<Vec<u64>>()
    }
}

impl<T: PrimeGeneration> PrimalityFromGeneration for T {}
impl<T: Primality> PrimeGenerationFromPrimality for T {}

/// Primality test by trivial division.
struct TrialDivision;

impl Primality for TrialDivision {
    fn is_prime(n: u64) -> bool {
        // 0 and 1 are not primes by convention.
        if n <= 1 { return false; }

        for i in 2..n {
            // We only need to check divisibility to the square root of n
            // rounded down. We are also avoiding float numbers here.
            if i > n / i { break; }
            if n % i == 0 { return false; }
        }
        true
    }
}

/// Primality test by Sieve of Eratosthenes.
struct FilteringSieve;

impl PrimeGeneration for FilteringSieve {
    fn primes_up_to(n: u64) -> Vec<u64> {
        // We start with the entire natural number line from 2 to n.
        let mut integer_line: Vec<u64> = (2..=n).collect();

        let mut i: usize = 0;
        while i < integer_line.len() {
            let x = integer_line[i];

            // We only need to check up to the floor of the square root of n.
            if x > n / x { break; }

            // Remove all multiples of x. We only need to check from the square
            // of x.
            let xx = x * x;
            integer_line.retain(|y| *y < xx || *y % x != 0);

            i += 1;
        }

        integer_line
    }
}

struct BooleanSieve;

impl PrimeGeneration for BooleanSieve {
    fn primes_up_to(n: u64) -> Vec<u64> {
        if n < 2 {
            return Vec::new();
        }

        let n = n as usize;
        let mut marked_primes = vec![true; n + 1];
        marked_primes[0] = false;
        marked_primes[1] = false;

        let mut i: usize = 2;

        while i <= n / i {
            if marked_primes[i] {
                let mut multiple = i * i; // we only have to start at the square power
                while multiple <= n {
                    marked_primes[multiple] = false;
                    multiple += i;
                }
            }
            i += 1;

        }

        marked_primes
            .iter()
            .enumerate()
            .filter_map(|(i, &prime)| {
                if prime {
                    Some(i as u64)
                } else {
                    None
                }
            })
            .collect()
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    const PRIMES_UP_TO_100: [u64; 25] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
        31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
        73, 79, 83, 89, 97,
    ];

    /// Hard-coded primality facts, independent of your generator implementations.
    fn assert_known_primality_cases<P: Primality>() {
        let known_primes = [
            2_u64, 3, 5, 7, 11, 13, 17, 19, 23, 29,
            31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
            73, 79, 83, 89, 97,
        ];

        let known_composites = [
            0_u64, 1, 4, 6, 8, 9, 10, 12, 14, 15,
            16, 18, 20, 21, 22, 24, 25, 26, 27, 28,
            30, 32, 33, 34, 35, 36, 38, 39, 40, 42,
            44, 45, 46, 48, 49, 50, 51, 52, 54, 55,
            56, 57, 58, 60, 62, 63, 64, 65, 66, 68,
            69, 70, 72, 74, 75, 76, 77, 78, 80, 81,
            82, 84, 85, 86, 87, 88, 90, 91, 92, 93,
            94, 95, 96, 98, 99, 100,
        ];

        for &n in &known_primes {
            assert!(
                P::is_prime(n),
                "Expected {} to be prime, but algorithm returned false",
                n
            );
        }

        for &n in &known_composites {
            assert!(
                !P::is_prime(n),
                "Expected {} to be composite/non-prime, but algorithm returned true",
                n
            );
        }
    }

    /// Checks exact outputs for a generator on several important inputs.
    fn assert_known_generation_cases<G: PrimeGeneration>() {
        assert_eq!(G::primes_up_to(0), vec![]);
        assert_eq!(G::primes_up_to(1), vec![]);
        assert_eq!(G::primes_up_to(2), vec![2]);
        assert_eq!(G::primes_up_to(3), vec![2, 3]);
        assert_eq!(G::primes_up_to(4), vec![2, 3]);
        assert_eq!(G::primes_up_to(10), vec![2, 3, 5, 7]);
        assert_eq!(G::primes_up_to(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
        assert_eq!(G::primes_up_to(100), PRIMES_UP_TO_100.to_vec());
    }

    /// Strong property test:
    /// for every n up to `limit`, the generator output must match the primes
    /// identified by TrialDivision.
    ///
    /// This is a good cross-check because it compares different implementations.
    fn assert_generator_matches_trial_division<G: PrimeGeneration>(limit: u64) {
        for n in 0..=limit {
            let expected = (2..=n)
                .filter(|&x| TrialDivision::is_prime(x))
                .collect::<Vec<u64>>();

            let got = G::primes_up_to(n);

            assert_eq!(
                got, expected,
                "Generator mismatch for n = {}.\nGot:      {:?}\nExpected: {:?}",
                n, got, expected
            );
        }
    }

    /// For a generator-derived primality test:
    /// compare its result against TrialDivision for every n up to `limit`.
    fn assert_primality_from_generation_matches_trial_division<G: PrimalityFromGeneration>(
        limit: u64,
    ) {
        for n in 0..=limit {
            let expected = TrialDivision::is_prime(n);
            let got = <G as PrimalityFromGeneration>::is_prime(n);

            assert_eq!(
                got, expected,
                "Derived primality mismatch for n = {}.\nGot:      {}\nExpected: {}",
                n, got, expected
            );
        }
    }

    /// For a primality-derived generator:
    /// compare its result against hard-coded / TrialDivision-backed expectations.
    fn assert_generation_from_primality_matches_trial_division<
        P: PrimeGenerationFromPrimality + Primality,
    >(
        limit: u64,
    ) {
        for n in 0..=limit {
            let expected = (2..=n)
                .filter(|&x| TrialDivision::is_prime(x))
                .collect::<Vec<u64>>();

            let got = <P as PrimeGenerationFromPrimality>::primes_up_to(n);

            assert_eq!(
                got, expected,
                "Derived generation mismatch for n = {}.\nGot:      {:?}\nExpected: {:?}",
                n, got, expected
            );
        }
    }

    // -------------------------------------------------------------------------
    // TrialDivision: direct primality
    // -------------------------------------------------------------------------

    #[test]
    fn trial_division_known_primality_cases() {
        assert_known_primality_cases::<TrialDivision>();
    }

    // -------------------------------------------------------------------------
    // TrialDivision: generation derived from primality
    // -------------------------------------------------------------------------

    #[test]
    fn trial_division_generation_from_primality_known_cases() {
        assert_eq!(
            <TrialDivision as PrimeGenerationFromPrimality>::primes_up_to(0),
            vec![]
        );
        assert_eq!(
            <TrialDivision as PrimeGenerationFromPrimality>::primes_up_to(1),
            vec![]
        );
        assert_eq!(
            <TrialDivision as PrimeGenerationFromPrimality>::primes_up_to(2),
            vec![2]
        );
        assert_eq!(
            <TrialDivision as PrimeGenerationFromPrimality>::primes_up_to(10),
            vec![2, 3, 5, 7]
        );
        assert_eq!(
            <TrialDivision as PrimeGenerationFromPrimality>::primes_up_to(100),
            PRIMES_UP_TO_100.to_vec()
        );
    }

    #[test]
    fn trial_division_generation_from_primality_matches_trial_division_up_to_200() {
        assert_generation_from_primality_matches_trial_division::<TrialDivision>(200);
    }

    // -------------------------------------------------------------------------
    // FilteringSieve: direct generation
    // -------------------------------------------------------------------------

    #[test]
    fn filtering_sieve_known_generation_cases() {
        assert_known_generation_cases::<FilteringSieve>();
    }

    #[test]
    fn filtering_sieve_matches_trial_division_up_to_200() {
        assert_generator_matches_trial_division::<FilteringSieve>(200);
    }

    // -------------------------------------------------------------------------
    // BooleanSieve: direct generation
    // -------------------------------------------------------------------------

    #[test]
    fn boolean_sieve_known_generation_cases() {
        assert_known_generation_cases::<BooleanSieve>();
    }

    #[test]
    fn boolean_sieve_matches_trial_division_up_to_200() {
        assert_generator_matches_trial_division::<BooleanSieve>(200);
    }

    // -------------------------------------------------------------------------
    // Derived primality from generation
    // -------------------------------------------------------------------------

    #[test]
    fn filtering_sieve_derived_primality_matches_trial_division_up_to_200() {
        assert_primality_from_generation_matches_trial_division::<FilteringSieve>(200);
    }

    #[test]
    fn boolean_sieve_derived_primality_matches_trial_division_up_to_200() {
        assert_primality_from_generation_matches_trial_division::<BooleanSieve>(200);
    }

    // -------------------------------------------------------------------------
    // Cross-check generators directly against each other
    // -------------------------------------------------------------------------

    #[test]
    fn filtering_and_boolean_sieves_agree_up_to_200() {
        for n in 0..=200 {
            let filtering = FilteringSieve::primes_up_to(n);
            let boolean = BooleanSieve::primes_up_to(n);

            assert_eq!(
                filtering, boolean,
                "FilteringSieve and BooleanSieve differ for n = {}.\nFiltering: {:?}\nBoolean:   {:?}",
                n, filtering, boolean
            );
        }
    }

    // -------------------------------------------------------------------------
    // Sanity checks on generator output shape
    // -------------------------------------------------------------------------

    fn assert_generator_output_is_sorted_and_unique<G: PrimeGeneration>(limit: u64) {
        for n in 0..=limit {
            let primes = G::primes_up_to(n);

            // Sorted strictly increasingly => unique and ordered.
            assert!(
                primes.windows(2).all(|w| w[0] < w[1]),
                "Output is not strictly increasing for n = {}: {:?}",
                n,
                primes
            );

            // Every reported number should actually be prime.
            assert!(
                primes.iter().all(|&p| TrialDivision::is_prime(p)),
                "Output contains a non-prime for n = {}: {:?}",
                n,
                primes
            );

            // Every reported number should be <= n.
            assert!(
                primes.iter().all(|&p| p <= n),
                "Output contains a value larger than n = {}: {:?}",
                n,
                primes
            );
        }
    }

    #[test]
    fn filtering_sieve_output_is_sorted_unique_and_valid() {
        assert_generator_output_is_sorted_and_unique::<FilteringSieve>(200);
    }

    #[test]
    fn boolean_sieve_output_is_sorted_unique_and_valid() {
        assert_generator_output_is_sorted_and_unique::<BooleanSieve>(200);
    }
}
