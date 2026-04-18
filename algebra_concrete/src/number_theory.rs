/// Traits for number theory.
pub mod traits;

/// greatest common divisor
pub mod gcd;

/// least common multple
pub mod lcm;

/// trial division
pub mod is_prime;

/// Sieve of Eratosthenes
pub mod sieve_of_eratosthenes;

pub use traits::RationalOps;
pub use gcd::*;
pub use lcm::*;
pub use is_prime::*;
pub use sieve_of_eratosthenes::*;