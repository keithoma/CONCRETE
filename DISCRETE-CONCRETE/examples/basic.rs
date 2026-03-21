trait Primality {
    /// Returns `true` if `n` is prime and `false` otherwise.
    fn is_prime(n: u64) -> bool;
}

trait PrimeGeneration {
    fn primes_up_to(n: u64) -> Result<Vec<u64>, String>;
}

/// Primality test by trivial division.
struct TrialDivision;

/// Primality test by Sieve of Eratosthenes.
struct SieveOfEratosthenes;

impl Primality for TrialDivision {
    fn is_prime(n: u64) -> bool {
        // 0 and 1 are not primes by convention.
        if n <= 1 {
            return false;
        }

        for i in 2..n {
            // We only need to check divisibility to the square root of n
            // rounded down. We are also avoiding float numbers here.
            if i > n / i {
                break;
            }

            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

impl Primality for SieveOfEratosthenes {
    fn is_prime(n: u64) -> bool {
        // 0 and 1 are not primes by convention.
        if n <= 1 {
            return false;
        }

        // We start with the entire natural number line from 2 to n.
        let mut integer_line: Vec<u64> = (2..=n).collect();

        let mut i: usize = 0;
        while i < integer_line.len() {
            let x = integer_line[i];

            // We only need to check up to the floor of the square root of n.
            if x > n / x {
                break;
            }

            // Remove all multiples of x. We only need to check from the square
            // of x.
            let xx = x * x;
            integer_line.retain(|y| *y < xx || *y % x != 0);

            i += 1;
        }

        // If the last remaining integer is `n`, then it is prime.
        integer_line.last() == Some(&n) 
    }
}

impl PrimeGeneration for TrialDivision {
    fn primes_up_to(n: u64) -> Result<Vec<u64>, String> {
        let mut set_of_primes: Vec<u64>;

        for i in 2..=n {
            if TrialDivision::Primality(i) {
                set_of_primes.push(i)
            }
        }

        set_of_primes;
    }
}



trait PrimeFactorizationAlgorithm {
    fn prime_factorization(a: u64) -> Result<Vec<(u64, u32)>, String>;
}

trait GCDAlgorithm {
    fn gcd(a: u64, b: u64) -> u64;
}

struct Euclidean;

impl GCDAlgorithm for Euclidean {
    fn gcd(mut a: u64, mut b: u64) -> u64 {
        a
    }
}

fn main() {
    // let a: u64 = 64;
    // println!("{}", TrivialDivision::is_prime(a));
    println!("{}", SieveOfEratosthenes::is_prime(14));
    println!("{}", SieveOfEratosthenes::is_prime(8));
    println!("{}", TrivialDivision::is_prime(310000001));

    // 2, 3, 5, 7, 11, 13, 17, 23, 29, 31
}