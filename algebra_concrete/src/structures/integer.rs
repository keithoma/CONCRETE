use core::cmp;
use core::ops;

use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Div;
use core::ops::Rem;

/// Integer
pub trait Integer: Sized + Copy + PartialEq + Eq + PartialOrd + Ord 
where 
    Self: 
        Add<Output = Self> + 
        Sub<Output = Self> + 
        Mul<Output = Self> + 
        Div<Output = Self> + 
        Rem<Output = Self>
{
    /// The additive identity (0)
    const ZERO: Self;

    /// The multiplicative identity (1)
    const ONE: Self;

    /// Returns the greatest common divisor of self and other.
    fn gcd(self, other: Self) -> Self;
}

impl Integer for i32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    fn gcd(self, other: Self) -> Self {

    }
}

impl Integer for u64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    fn gcd(self, other: Self) -> Self {
        // We use the full path to reach your crate's gcd module
        crate::number_theory::gcd_u64::gcd(self, other)
    }
}