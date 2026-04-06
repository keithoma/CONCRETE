/// Traits for number theory.
pub mod traits;

/// greatest common divisor
pub mod gcd;

/// least common multple
pub mod lcm;

/// all about primes
pub mod is_prime;

pub use traits::RationalOps;
pub use gcd::*;
pub use lcm::*;