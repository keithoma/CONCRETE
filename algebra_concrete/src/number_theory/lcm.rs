//! least common multiple

use core::marker::PhantomData;

use crate::structures::integer::Integer;
use crate::number_theory::gcd::GcdAlgorithm;

pub trait LcmAlgorithm<T: Integer> {
    fn compute(a: T, b: T) -> T;
}

// The struct takes a generic type G representing the GCD strategy
pub struct FormulaicGcd<G>(PhantomData<G>);

// We bound G to ensure it is a valid GcdAlgorithm for our integer T
impl<T, G> LcmAlgorithm<T> for FormulaicGcd<G> 
where 
    T: Integer,
    G: GcdAlgorithm<T> 
{
    #[inline]
    fn compute(a: T, b: T) -> T {
        if a.is_zero() || b.is_zero() { return T::ZERO; }
        
        // We bypass a.gcd() and explicitly inject the user's chosen strategy!
        (a / G::compute(a, b)) * b
    }
}
