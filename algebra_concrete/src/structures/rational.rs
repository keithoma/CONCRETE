
use core::ops::Neg;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Sub;

use crate::structures::integer::Integer;

/// Represents a rational number.
#[derive(Debug)]
pub struct Rational<T: Integer> {
    /// numerator
    num: T,

    /// denominator
    den: T,
}

impl<T: Integer> Rational<T> {
    /// a rational
    /// 
    /// # Panics
    pub fn new(mut num: T, mut den: T) -> Self {
        assert!(den != T::ZERO, "Denominator cannot be zero.");

        let gcd = num.gcd(den);

        #[expect(
            clippy::arithmetic_side_effects,
            reason = "Verified non-zero denominator; GCD division and sign flip are within bounds."
        )]
        if den.is_negative() {
            num = (T::ZERO - num) / gcd;
            den = (T::ZERO - den) / gcd;
        } else {
            num /= gcd;
            den /= gcd;
        }

        Self { num, den }
    }
}

impl<T: Integer> Neg for Rational<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(T::ZERO - self.num, self.den)
    }
}

impl<T: Integer> Add for Rational<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let lcm = self.den.lcm(other.den);
        let num = self.num * (lcm / self.den) + other.num * (lcm / other.den);
        Self::new(num, lcm)
    }
}

impl<T: Integer> AddAssign for Rational<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self + rhs; // we don't need to create another rational object
    }
}

impl<T: Integer> Sub for Rational<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let lcm = self.den.lcm(other.den);
        let num = self.num * (lcm / self.den) - other.num * (lcm / other.den);
        Self::new(num, lcm)
    }
}