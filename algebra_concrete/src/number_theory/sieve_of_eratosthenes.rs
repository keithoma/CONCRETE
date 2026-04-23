//! the greek guy

use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryFrom;

use crate::{structures::integer::Integer};

/// Sieve of Eratosthenes
#[derive(Debug)] #[non_exhaustive] pub struct SieveOfEratosthenes;

pub trait PrimeAlgorithm2<T: Integer> {
    /// computes the gcd
    fn is_prime(n: T) -> bool;

    /// outputs all the primes
    fn primes_up_to(n: T) -> Vec<T>;
}

impl<T> PrimeAlgorithm2<T> for SieveOfEratosthenes
where
    T: Integer + TryFrom<usize>,
    usize: TryFrom<T>,
{
    fn primes_up_to(n: T) -> Vec<T> {
        // 0 and 1 are not primes by convention.
        if n == T::ZERO || n == T::ONE { return Vec::new() }

        let n_usize: usize = n.to_usize();
        let mut number_line = vec![true; n_usize - 1]; // we check from 2 to n
        let mut i = 2; // we start from 2

        // we need only to check up to the floor of sqrt(n)
        while i <= n_usize / i {
            // remember that the index of the vector has an offset of 2
            if number_line[i - 2] {
                let j = i * i;
                while j <= n_usize {
                    number_line[j - 2] = false;
                    j += i;
                }
            }

            i += 1;
        }

        number_line.iter()
            .enumerate()
            .filter(|(_, &val)| val)
            .map(|(index, _)| {
                let prime_usize = index + 2;
                // Convert back from usize to T
                T::try_from(prime_usize).ok().expect("Value fits in usize but not T")
            })
            .collect()
    }

    fn is_prime(n: T) -> bool {
        !todo()
    }
}
