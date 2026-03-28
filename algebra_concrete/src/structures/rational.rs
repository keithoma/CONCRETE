use core::ops::Add;

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

// TODO: doesn't quite work lol
impl<T: Integer> Add for Rational<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let gcd = self.den.lcm(other.den);
        let lcm = self.den.lcm(other.den);
        Self {
            num: self.num * gcd + other.num * gcd,
            den: lcm,
        }
    }
}