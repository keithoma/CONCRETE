//! least common multiple

use core::marker::PhantomData;

use crate::structures::integer::Integer;
use crate::number_theory::gcd::GcdAlgorithm;

/// trait for lcm algorithms
pub trait LcmAlgorithm<T: Integer> {
    /// computes the lcm
    fn compute(a: T, b: T) -> T;
}

/// The struct takes a generic type G representing the GCD strategy
#[derive(Debug)] pub struct FormulaicGcd<G>(PhantomData<G>);

// We bound G to ensure it is a valid GcdAlgorithm for our integer T
impl<T, G> LcmAlgorithm<T> for FormulaicGcd<G> 
where 
    T: Integer,
    G: GcdAlgorithm<T> 
{
    
    #![expect(clippy::arithmetic_side_effects, reason = "it's safe I think")]
    #[inline]
    fn compute(a: T, b: T) -> T {
        if a.is_zero() || b.is_zero() { return T::ZERO; }
        
        (a / G::compute(a, b)) * b
    }
}
