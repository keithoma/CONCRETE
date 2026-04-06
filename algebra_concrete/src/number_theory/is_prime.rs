//! all about primes
//! thinking if I should split is_prime(n) and primes_up_to(n) into two files, anyway

use crate::structures::integer::{Integer};

/// strategy for iterative euclidean algorithm
#[derive(Debug)] #[non_exhaustive] pub struct TrialDivision;

/// primes
pub trait PrimeAlgorithm<T: Integer> {
    /// computes the gcd
    fn is_prime(n: T) -> bool;

    // maybe primes_up_to here?
}

impl<T: Integer> PrimeAlgorithm<T> for TrialDivision {
    fn is_prime(n: T) -> bool {
        #![expect(
            clippy::arithmetic_side_effects, 
            reason = "none, maybe they break idk"
        )]

        // 0 and 1 are not primes by convention.
        if n == T::ZERO || n == T::ONE { return false; }

        // We iterate from 2 to the the square root of `n` rounded down.
        let mut i = T::ONE + T::ONE;
        while i <= n / i {
            if n % i == T::ZERO {
                return false;
            }
            i += T::ONE;
        }
        true
    }
}
