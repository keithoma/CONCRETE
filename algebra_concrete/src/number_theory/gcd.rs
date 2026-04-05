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

// ISSUES: in stable Rust, we cannot call trait methods isnide a const fn

// TODO: add a wrapper functions for gcd so that it works for unsigned ints

use crate::structures::integer::{BitwiseOps, Integer};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum EuclideanGcdStrategy {
    /// Iterative Euclidean Algorithm.
    /// The standard approach using the modulus operator in a loop.
    #[default] Iterative,

    /// Recursive Euclidean Algorithm.
    /// The standard modulus approach using tail recursion.
    Recursive,

    /// Classical Euclidean Algorithm via repeated subtraction.
    /// The original "Greek" method; significantly slower for numbers with large differences.
    Subtraction,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum SteinGcdStrategy {
    /// Iterative Binary GCD (Stein's Algorithm).
    /// Optimized for modern CPUs using shifts and count-trailing-zeros.
    #[default] Iterative,
    
    /// Recursive Binary GCD (Stein's Algorithm).
    /// Uses structural recursion; primarily for educational or verification use.
    Recursive,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum GcdStrategy {
    #[default] Euclidean(EuclideanGcdStrategy),
    Stein(SteinGcdStrategy),
}

#[inline]
pub fn gcd<T: Integer>(a: T, b: T) -> T {
    gcd_with_strategy(a, b, GcdStrategy::default())
}

pub fn gcd_with_strategy<T: Integer>(a: T, b: T, strategy: GcdStrategy) -> T {
    match strategy {
        GcdStrategy::EuclideanIterative => euclidean_iterative(a, b),
        GcdStrategy::EuclideanSubtraction => euclidean_subtraction(a, b),
        GcdStrategy::EuclideanRecursive => euclidean_recursive(a, b),
    }
}

#[inline]
pub fn gcd_binary<T: Integer + BitwiseOps>(a: T, b: T) -> T {
    gcd_binary_with_strategy(a, b, GcdStrategy::SteinIterative)
}

pub fn gcd_binary_with_strategy<T: Integer + BitwiseOps>(a: T, b:T, strategy: GcdStrategy) -> T {
    match strategy {
        GcdStrategy::SteinIterative => stein_iterative(a, b),
        GcdStrategy::SteinRecursive => stein_recursive(a, b),
        GcdStrategy::EuclideanIterative => euclidean_iterative(a, b),
        GcdStrategy::EuclideanSubtraction => euclidean_subtraction(a, b),
        GcdStrategy::EuclideanRecursive => euclidean_recursive(a, b),
    }
}

/// Computes the GCD using the iterative Stein's Algorithm (Binary GCD).
///
/// This implementation is specialized for unsigned integers, avoiding division by
/// using arithmetic shifts and subtractions.
///
/// # Examples
///
/// ```rust
/// assert_eq!(stein_iterative(48u32, 18u32), 6);
/// assert_eq!(stein_iterative(0u32, 5u32), 5);
/// ```
///
/// # Implementation
///
/// The algorithm follows these logical steps:
/// 1. Handle identity cases where `a` or `b` is zero.
/// 2. Find the common power of 2 by comparing trailing zeros.
/// 3. Shift out all powers of 2 to make both numbers odd.
/// 4. Iteratively subtract the smaller from the larger and shift until `b` is 0.
/// 5. Re-apply the common power of 2 to the result.
///
/// * Time Complexity: O(n^2) where n is the number of bits.
/// * Space Complexity: O(1) auxiliary.
fn stein_iterative<T: Integer + BitwiseOps>(mut a: T, mut b: T) -> T {
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

/// Computes the GCD using the recursive Stein's Algorithm (Binary GCD).
///
/// This implementation follows the binary GCD logic through structural recursion,
/// identifying common factors of 2 and reducing odd numbers via subtraction.
///
/// # Examples
///
/// ```rust
/// assert_eq!(stein_recursive(48u32, 18u32), 6);
/// assert_eq!(stein_recursive(7u32, 13u32), 1);
/// ```
///
/// # Implementation
///
/// The algorithm uses a match-based state machine to handle bitwise reduction:
/// 1. Base cases: If either `a` or `b` is 0, the other is the GCD.
/// 2. Both even: `gcd(a/2, b/2) * 2`.
/// 3. One even, one odd: `gcd(even/2, odd)`.
/// 4. Both odd: `gcd(|a-b|/2, min(a,b))`.
///
/// * Time Complexity: O(n^2) bit operations where n is the number of bits.
/// * Space Complexity: O(n) stack frames due to recursion depth.
fn stein_recursive<T: Integer + BitwiseOps>(a: T, b: T) -> T {
    if a.is_zero() {
        return b;
    }
    if b.is_zero() {
        return a;
    }

    match (a & T::ONE == T::ZERO, b & T::ONE == T::ZERO) {
        (true, true) => stein_recursive(a >> 1, b >> 1) << 1,
        (true, false) => stein_recursive(a >> 1, b),
        (false, true) => stein_recursive(a, b >> 1),
        (false, false) => {
            if a <= b {
                stein_recursive((b - a) >> 1, a)
            } else {
                stein_recursive((a - b) >> 1, b)
            }
        },
    }
}

/// Computes the GCD using the iterative Euclidean Algorithm.
///
/// This implementation uses the modulo operator to reduce the numbers until the
/// remainder is zero.
///
/// # Examples
///
/// ```rust
/// assert_eq!(euclidean_iterative(48u32, 18u32), 6);
/// assert_eq!(euclidean_iterative(101u32, 103u32), 1);
/// ```
///
/// # Implementation
///
/// The algorithm repeatedly applies the property gcd(a, b) = gcd(b, a % b):
/// 1. Loop while the divisor `b` is not zero.
/// 2. Compute the remainder of `a` divided by `b`.
/// 3. Update `a` with the previous divisor and `b` with the remainder.
/// 4. Return `a` once `b` reaches zero.
///
/// * Time Complexity: O(n^2) where n is the number of bits.
/// * Space Complexity: O(1) auxiliary space.
fn euclidean_iterative<T: Integer>(mut a: T, mut b: T) -> T {
    while b.is_nonzero() {
        (a, b) = (b, a % b);
    }
    a
}

/// Computes the GCD using the Euclidean Algorithm via repeated subtraction.
///
/// This is the classical "Greek" approach (anthyphairesis), which avoids division
/// and modulus by repeatedly subtracting the smaller value from the larger.
///
/// # Examples
///
/// ```rust
/// assert_eq!(euclidean_subtraction(48u32, 18u32), 6);
/// assert_eq!(euclidean_subtraction(7u32, 13u32), 1);
/// ```
///
/// # Implementation
///
/// The algorithm proceeds until both numbers are equal:
/// 1. Handle identity cases where `a` or `b` is zero.
/// 2. While `a` and `b` are not equal, subtract the smaller from the larger.
/// 3. Return the remaining value once equality is reached.
///
/// * Time Complexity: O(max(a, b)) in the worst case (e.g., gcd(n, 1)).
/// * Space Complexity: O(1) auxiliary space.
fn euclidean_subtraction<T: Integer>(mut a: T, mut b: T) -> T {
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

/// Computes the GCD using the recursive Euclidean Algorithm.
///
/// This implementation reduces the problem size by taking the remainder of `a`
/// divided by `b` in each recursive step until the base case of zero is reached.
///
/// # Examples
///
/// ```rust
/// assert_eq!(euclidean_recursive(48u32, 18u32), 6);
/// assert_eq!(euclidean_recursive(101u32, 103u32), 1);
/// ```
///
/// # Implementation
///
/// The algorithm relies on the recurrence gcd(a, b) = gcd(b, a % b):
/// 1. Base case: If `b` is 0, then `a` is the GCD.
/// 2. Recursive step: Calculate the remainder of `a` / `b`.
/// 3. Tail call: Recurse with `b` as the new `a`, and the remainder as the new `b`.
///
/// * Time Complexity: O(n^2) where n is the number of bits.
/// * Space Complexity: O(n) stack frames (logarithmic relative to the value).
fn euclidean_recursive<T: Integer>(a: T, b: T) -> T {
    if b.is_nonzero() {
        euclidean_recursive(b, a % b)
    } else {
        a
    }
}

#[cfg(test)]
mod test;
