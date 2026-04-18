//! the greek guy

use alloc::vec;
use alloc::vec::Vec;

use crate::{structures::integer::Integer};

/// Sieve of Eratosthenes
#[derive(Debug)] #[non_exhaustive] pub struct SieveOfEratosthenes;

pub trait PrimeAlgorithm2<T: Integer> {
    /// computes the gcd
    fn is_prime(n: T) -> bool;

    /// outputs all the primes
    fn primes_up_to(n: T) -> Vec<T>;
}

impl<T: Integer> PrimeAlgorithm2<T> for SieveOfEratosthenes {
    fn primes_up_to(n: T) -> Vec<T> {
        // 0 and 1 are not primes by convention.
        if n == T::ZERO || n == T::ONE { return Vec::new() }
        let vec_size = (n - T::ONE).to_usize();

        let mut number_line = vec![true; vec_size]; // we check from 2 to n
        let two = T::ONE + T::ONE; // just a helper constant
        let mut i = two; // we start from 2

        // we need only to check up to the floor of sqrt(n)
        while i <= n / i {
            // remember that the index of the vector has an offset of 2
            if number_line[i - two] {
                let j = i;
                while j < n + T::ONE {
                    number_line[j] = false;
                    j += i;
                }
            }

            i += T::ONE;
        }

        return number_line.iter()
            .enumerate()
            .filter(|(_, &val)| val)
            .map(|(i, _)| i + two)
            .collect()
    }

    fn is_prime(n: T) -> bool {
        !todo()
    }
}
