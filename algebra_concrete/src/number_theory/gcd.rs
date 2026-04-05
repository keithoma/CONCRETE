//! Arithmetic utility for computing the Greatest Common Divisor (GCD).
//!
//! This module provides multiple implementations of the GCD algorithm, ranging from classical
//! Euclidean methods to hardware-optimized binary (Stein's) algorithms.
//!
//! # Main Entry Point
//!
//! For most use cases, the [`gcd`] function is the recommended entry point as it defaults
//! to the most efficient strategy for the underlying integer type.
//!
//! # Performance Note
//!
//! Different strategies have different performance characteristics based on the input size
//! and CPU architecture (e.g., availability of the `ctz` instruction).

// TODO: add a wrapper functions for gcd so that it works for unsigned ints

use crate::structures::integer::{BitwiseOps, Integer};

pub struct EuclideanIterative;
pub struct EuclideanSubtraction;
pub struct EuclideanRecursive;
pub struct SteinIterative;
pub struct SteinRecursive;

pub trait GcdAlgorithm<T: Integer> {
    fn compute(a: T, b: T) -> T;
}

impl<T: Integer> GcdAlgorithm<T> for EuclideanIterative {
    #[inline]
    fn compute(mut a: T, mut b: T) -> T {
        while b.is_nonzero() {
            (a, b) = (b, a % b);
        }
        a
    }
}

impl<T: Integer> GcdAlgorithm<T> for EuclideanSubtraction {
    #[inline]
    fn compute(mut a: T, mut b: T) -> T {
        if a.is_zero() {
            return b;
        }
        if b.is_zero() {
            return a;
        }

        while a != b {
            if a > b {
                a -= b
            } else {
                b -= a
            }
        }

        a
    }
}

impl<T: Integer> GcdAlgorithm<T> for EuclideanRecursive {
    #[inline]
    fn compute(a: T, b: T) -> T {
        if b.is_nonzero() {
            Self::compute(b, a % b)
        } else {
            a
        }
    }
}

impl<T: Integer + BitwiseOps> GcdAlgorithm<T> for SteinIterative {
    #[inline]
    fn compute(mut a: T, mut b: T) -> T {
        if a.is_zero() {
            return b;
        }
        if b.is_zero() {
            return a;
        }

        let a_zeros = a.trailing_zeros();
        a >>= a_zeros;

        let b_zeros = b.trailing_zeros();
        b >>= b_zeros;

        let common_zeros = if a_zeros < b_zeros { a_zeros } else { b_zeros };

        loop {
            if a > b {
                (a, b) = (b, a);
            }

            b -= a;

            if b.is_zero() {
                break;
            }

            b >>= b.trailing_zeros();
        }

        a << common_zeros
    }
}

impl<T: Integer + BitwiseOps> GcdAlgorithm<T> for SteinRecursive {
    #[inline]
    fn compute(a: T, b: T) -> T {
        if a.is_zero() {
            return b;
        }
        if b.is_zero() {
            return a;
        }

        match (a & T::ONE == T::ZERO, b & T::ONE == T::ZERO) {
            (true, true) => Self::compute(a >> 1_u32, b >> 1_u32) << 1_u32,
            (true, false) => Self::compute(a >> 1_u32, b),
            (false, true) => Self::compute(a, b >> 1_u32),
            (false, false) => {
                if a <= b {
                    Self::compute((b - a) >> 1_u32, a)
                } else {
                    Self::compute((a - b) >> 1_u32, b)
                }
            },
        }
    }
}
