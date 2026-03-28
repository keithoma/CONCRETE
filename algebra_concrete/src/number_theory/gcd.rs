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

// TODO: ``lcm()``
// TODO: negative integer handeling
// TODO: trait achitecture

/// Strategies available for computing the Greatest Common Divisor.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum GcdStrategy {
    /// Iterative Binary GCD (Stein's Algorithm). 
    /// Optimized for modern CPUs using shifts and count-trailing-zeros.
    #[default]
    SteinIterative,

    /// Recursive Binary GCD (Stein's Algorithm). 
    /// Uses structural recursion; primarily for educational or verification use.
    SteinRecursive,

    /// Iterative Euclidean Algorithm. 
    /// The standard approach using the modulus operator in a loop.
    EuclideanIterative,

    /// Recursive Euclidean Algorithm. 
    /// The standard modulus approach using tail recursion.
    EuclideanRecursive,

    /// Classical Euclidean Algorithm via repeated subtraction. 
    /// The original "Greek" method; significantly slower for numbers with large differences.
    EuclideanSubtraction,
}

/// Internal macro to implement GCD functions for unsigned integer primitives.
///
/// This macro generates a submodule for the specified type, containing both the 
/// default [`gcd`] function and the [`gcd_with_strategy`] variant.
macro_rules! impl_unsigned_gcd {
    ($t:ty, $mod_name:ident) => {
        /// docstring
        pub mod $mod_name {

            use super::GcdStrategy;

            /// Returns the greatest common divisor (GCD) of two numbers.
            ///
            /// The GCD is the largest positive integer that divides both `a` and `b` without
            /// a remainder. In ring theory, this is the unique non-negative generator of the
            /// ideal `aZ + bZ`.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use crate::$mod_name::gcd;
            /// assert_eq!(gcd(48, 18), 6);
            /// assert_eq!(gcd(101, 103), 1);
            /// assert_eq!(gcd(0, 5), 5);
            /// assert_eq!(gcd(0, 0), 0);
            /// ```
            ///
            /// # Mathematical Properties
            ///
            /// * Commutativity: `gcd(a, b) == gcd(b, a)`
            /// * Identity: `gcd(a, 0) == a`
            /// * LCM Relation: `gcd(a, b) * lcm(a, b) == a * b`
            ///
            /// # Implementation
            ///
            /// This function uses Stein's Algorithm (Binary GCD). It replaces standard Euclidean
            /// division with arithmetic shifts and subtractions, leveraging the `ctz` (count
            /// trailing zeros) instruction for O(1) power-of-2 extraction.
            ///
            /// * Time Complexity: O(n^2) bit operations, where n is the number of bits.
            /// * Space Complexity: O(1) auxiliary space.
            pub const fn gcd(a: $t, b: $t) -> $t {
                stein_iterative(a, b)
            }

            /// Returns the greatest common divisor (GCD) of two numbers using a specific
            /// [`GcdStrategy`].
            ///
            /// This function provides the same mathematical result as [`gcd`], but allows
            /// manual selection of the underlying algorithm. This is useful for benchmarking
            /// or specialized hardware constraints.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use crate::$mod_name::{gcd_with_strategy, GcdStrategy};
            /// let result = gcd_with_strategy(48, 18, GcdStrategy::EuclideanIterative);
            /// assert_eq!(result, 6);
            /// ```
            ///
            /// # Strategies
            ///
            /// * [`GcdStrategy::SteinIterative`]: Binary GCD. Efficient; uses shifts and `ctz`.
            /// * [`GcdStrategy::SteinRecursive`]: Binary GCD using recursion.
            /// * [`GcdStrategy::EuclideanIterative`]: Standard modulus-based algorithm using a
            ///   loop.
            /// * [`GcdStrategy::EuclideanRecursive`]: Standard modulus-based algorithm using
            ///   recursion.
            /// * [`GcdStrategy::EuclideanSubtraction`]: The original Greek approach using repeated
            ///   subtraction. Slower, but demonstrates the fundamental logic of the ideal.
            ///
            /// For detailed mathematical properties and complexity analysis, see the [`gcd`]
            /// function.
            pub const fn gcd_with_strategy(a: $t, b: $t, strategy: GcdStrategy) -> $t {
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
            /// # use crate::$mod_name::stein_iterative;
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
            pub(crate) const fn stein_iterative(mut a: $t, mut b: $t) -> $t {
                if a == 0 {
                    return b;
                }
                if b == 0 {
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
                    if let Some(diff) = b.checked_sub(a) {
                        b = diff;
                    }
                    if b == 0 {
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
            /// # use crate::$mod_name::stein_recursive;
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
            pub(crate) const fn stein_recursive(a: $t, b: $t) -> $t {
                match (a, b) {
                    (0, y) => y,
                    (x, 0) => x,
                    (x, y) => match (x & (1 as $t) == 0, y & (1 as $t) == 0) {
                        (true, true) => stein_recursive(x >> 1, y >> 1) << 1,
                        (true, false) => stein_recursive(x >> 1, y),
                        (false, true) => stein_recursive(x, y >> 1),
                        (false, false) => {
                            if x >= y {
                                match x.checked_sub(y) {
                                    Some(diff) => stein_recursive(diff >> 1, y),
                                    None => x,
                                }
                            } else {
                                match y.checked_sub(x) {
                                    Some(diff) => stein_recursive(diff >> 1, x),
                                    None => y,
                                }
                            }
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
            /// # use crate::$mod_name::euclidean_iterative;
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
            pub(crate) const fn euclidean_iterative(mut a: $t, mut b: $t) -> $t {
                while b != 0 {
                    if let Some(rem) = a.checked_rem(b) {
                        a = b;
                        b = rem;
                    }
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
            /// # use crate::$mod_name::euclidean_subtraction;
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
            pub(crate) const fn euclidean_subtraction(mut a: $t, mut b: $t) -> $t {
                if a == 0 {
                    return b;
                }
                if b == 0 {
                    return a;
                }

                while a != b {
                    if let Some(diff) = a.checked_sub(b) {
                        a = diff;
                    } else if let Some(diff) = b.checked_sub(a) {
                        b = diff;
                    } else {
                        break;
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
            /// # use crate::$mod_name::euclidean_recursive;
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
            pub(crate) const fn euclidean_recursive(a: $t, b: $t) -> $t {
                // Formatting adjusted slightly for macro evaluation
                if b != 0 {
                    if let Some(rem) = a.checked_rem(b) {
                        return euclidean_recursive(b, rem);
                    }
                }
                a
            }

            #[cfg(test)]
            mod tests {
                extern crate std; // needed for println!() and I don't like it

                use super::*;

                const MAX: $t = <$t>::MAX;
                const CASES: [($t, $t, $t); 6] = [
                    (0, 0, 0),
                    (0, MAX, MAX),
                    (MAX, 0, MAX),
                    (MAX, MAX, MAX),
                    (48, 18, 6),
                    (17, 7, 1),
                ];

                type FuncDef = fn($t, $t) -> $t;
                const FUNCTIONS: &[(&str, FuncDef)] = &[
                    ("iterative Stein's algorithm", stein_iterative),
                    ("recursive Stein's algorithm", stein_recursive),
                    ("Iterative", euclidean_iterative),
                    ("Subtraction", euclidean_subtraction),
                    ("Recursive", euclidean_recursive),
                ];

                #[test]
                fn test_all() {
                    for (a, b, expected) in CASES {
                        assert_eq!(
                            gcd_with_strategy(a, b, GcdStrategy::EuclideanRecursive), expected
                        );
                        for (name, func) in FUNCTIONS {
                            // std::println!(
                            //     "Testing {name} implementation for the inputs {a} and {b}."
                            // );
                            let result = func(a, b);
                            assert_eq!(
                                result, expected,
                                "{name} failed for gcd({a}, {b}). Expected {expected}, got {result}"
                            );
                        }
                    }
                }
            }
        }
    };
}


macro_rules! impl_signed_gcd {
    ($t_signed:ty, $t_unsigned:ty, $signed_mod:ident, $unsigned_mod:ident) => {
        pub mod $signed_mod {
            use super::GcdStrategy;
            use super::$unsigned_mod as unsigned;

            pub const fn gcd(a: $t_signed, b: $t_signed) -> $t_unsigned {
                unsigned::gcd(a.unsigned_abs(), b.unsigned_abs())
            }

            pub const fn gcd_with_strategy(
                a: $t_signed, 
                b: $t_signed, 
                strategy: GcdStrategy
            ) -> $t_unsigned {
                unsigned::gcd_with_strategy(a.unsigned_abs(), b.unsigned_abs(), strategy)
            }

            pub(crate) const fn stein_iterative(a: $t_signed, b: $t_signed) -> $t_unsigned {
                unsigned::stein_iterative(a.unsigned_abs(), b.unsigned_abs())
            }

            pub(crate) const fn stein_recursive(a: $t_signed, b: $t_signed) -> $t_unsigned {
                unsigned::stein_recursive(a.unsigned_abs(), b.unsigned_abs())
            }

            pub(crate) const fn euclidean_iterative(a: $t_signed, b: $t_signed) -> $t_unsigned {
                unsigned::euclidean_iterative(a.unsigned_abs(), b.unsigned_abs())
            }

            pub(crate) const fn euclidean_subtraction(a: $t_signed, b: $t_signed) -> $t_unsigned {
                unsigned::euclidean_subtraction(a.unsigned_abs(), b.unsigned_abs())

            }
            pub(crate) const fn euclidean_recursive(a: $t_signed, b: $t_signed) -> $t_unsigned {
                unsigned::euclidean_recursive(a.unsigned_abs(), b.unsigned_abs())
            }

            #[cfg(test)]
            mod tests {
                extern crate std; // needed for println!() and I don't like it

                use super::*;

                const MAX: $t_signed = <$t_signed>::MAX;
                const MAX_UNSINGED: $t_unsigned = <$t_unsigned>::MAX;
                const CASES: [($t_signed, $t_signed, $t_unsigned); 6] = [
                    (-1, 0, 1),
                    (0, -MAX, MAX_UNSINGED),
                    (-MAX, 0, MAX_UNSINGED),
                    (-MAX, -MAX, MAX_UNSINGED),
                    (48, -18, 6),
                    (-17, -7, 1),
                ];

                type FuncDef = fn($t_signed, $t_signed) -> $t_unsigned;
                const FUNCTIONS: &[(&str, FuncDef)] = &[
                    ("gcd", gcd),
                    ("iterative Stein's algorithm", stein_iterative),
                    ("recursive Stein's algorithm", stein_recursive),
                    ("iterative Euclidean algorithm", euclidean_iterative),
                    ("Euclidean algorithm with subtraction", euclidean_subtraction),
                    ("recursive Euclidean algorithm", euclidean_recursive),
                ];

                #[test]
                fn test_all() {
                    for (a, b, expected) in CASES {
                        assert_eq!(
                            gcd_with_strategy(a, b, GcdStrategy::EuclideanRecursive), expected
                        );

                        for (name, func) in FUNCTIONS {
                            // std::println!(
                            //     "Testing {name} implementation for the inputs {a} and {b}."
                            // );
                            let result = func(a, b);
                            assert_eq!(
                                result, expected,
                                "{name} failed for gcd({a}, {b}). Expected {expected}, got {result}"
                            );
                        }
                    }
                }
            }
        }
    }
}


impl_unsigned_gcd!(u8, gcd_u8);
impl_unsigned_gcd!(u16, gcd_u16);
impl_unsigned_gcd!(u32, gcd_u32);
impl_unsigned_gcd!(u64, gcd_u64);
impl_unsigned_gcd!(u128, gcd_u128);
impl_unsigned_gcd!(usize, gcd_usize);

impl_signed_gcd!(i8, u8, gcd_i8, gcd_u8);
impl_signed_gcd!(i16, u16, gcd_i16, gcd_u16);
impl_signed_gcd!(i32, u32, gcd_i32, gcd_u32);
impl_signed_gcd!(i64, u64, gcd_i64, gcd_u64);
impl_signed_gcd!(i128, u128, gcd_i128, gcd_u128);