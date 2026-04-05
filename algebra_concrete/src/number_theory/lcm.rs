//! least common multiple

use crate::structures::integer::{BitwiseOps, Integer};
use crate::number_theory::gcd;

#[non_exhaustive]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum LcmStrategy {
    #[default] UsingGcd,
}

pub fn lcm<T: Integer + BitwiseOps>(a: T, b: T) -> T {
    using_gcd(a, b)
}

#[inline]
pub fn using_gcd<T: Integer + BitwiseOps>(a: T, b: T) -> T {
    a * b / gcd(a, b)
}