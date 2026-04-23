//! the greek guy

use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryFrom;

use crate::{structures::integer::Integer};

/// Sieve of Eratosthenes
#[derive(Debug)] #[non_exhaustive] pub struct SieveOfEratosthenes;

/// we really have to refactor this
pub trait PrimeAlgorithm2<T: Integer> {
    /// computes the gcd
    fn is_prime(n: T) -> bool;

    /// outputs all the primes
    fn primes_up_to(n: T) -> Vec<T>;
}

impl<T> PrimeAlgorithm2<T> for SieveOfEratosthenes
where
    T: Integer<UsizeType = usize> + TryFrom<usize>,
    usize: TryFrom<T>,
{
    #[expect(
        clippy::arithmetic_side_effects, 
        clippy::integer_division,
        clippy::indexing_slicing,
        clippy::expect_used,
        reason = "I'm lazy"
    )]
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
                let mut j = i * i;
                while j <= n_usize {
                    number_line[j - 2] = false;
                    j += i;
                }
            }

            i += 1;
        }

        number_line.iter()
            .enumerate() // Returns (usize, &bool)
            .filter(|&(_, val)| *val) // Destructure the reference to the bool
            .map(|(index, _)| {
                let prime_usize = index + 2;
                T::try_from(prime_usize).ok().expect("Failed to convert prime back to T")
                })
            .collect()
    }

    fn is_prime(n: T) -> bool {
        Some(n) == Self::primes_up_to(n).last().copied()
    }
}
