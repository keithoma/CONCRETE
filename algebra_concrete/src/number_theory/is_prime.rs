//! all about trial_division

#![expect(clippy::arithmetic_side_effects)]

use alloc::vec::Vec;

use crate::{structures::integer::Integer};

/// trial division
#[derive(Debug)] #[non_exhaustive] pub struct TrialDivision;

/// primes
pub trait PrimeAlgorithm<T: Integer> {
    /// computes the gcd
    fn is_prime(n: T) -> bool;

    /// outputs all the primes
    fn primes_up_to(n: T) -> Vec<T>;
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

    fn primes_up_to(n: T) -> Vec<T> {
        let mut primes_list: Vec<T> = Vec::new();
        let mut i: T = T::ONE + T::ONE;

        while i <= n {
            if Self::is_prime(i) {
                primes_list.push(i);
            }
            i += T::ONE;
        }
        
        primes_list
    }
}
