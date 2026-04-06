//! all about primes
//! thinking if I should split is_prime(n) and primes_up_to(n) into two files, anyway

use crate::structures::integer::{Integer};

/// strategy for iterative euclidean algorithm
#[derive(Debug)] #[non_exhaustive] pub struct TrialDivision;

pub trait PrimeAlgorithm<T: Integer> {
    /// computes the gcd
    fn is_prime(n: T) -> bool;

    // maybe primes_up+to here?
}

impl<T: Integer> PrimeAlgorithm<T> for TrialDivision {
    fn is_prime(n: T) -> bool {
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