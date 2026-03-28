//!
//! # To-do
//! [] implement ``lcm()``
//! [] implement negative integer handeling
//! [] implement trait achitecture

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum GcdStrategy {
    #[default]
    SteinIterative,
    SteinRecursive,
    EuclideanIterative,
    EuclideanSubtraction,
    EuclideanRecursive,
}

macro_rules! impl_unsigned_gcd {
    ($t:ty, $mod_name:ident) => {
        /// docstring
        pub mod $mod_name {

            use super::GcdStrategy;
            use super::GcdStrategy::*;

            /// Computes the greatest common divisor $\gcd(a, b)$ of two unsigned integers.
            ///
            /// This implementation defaults to **Stein's algorithm** (Binary GCD), which replaces 
            /// the computationally expensive remainder operations of the Euclidean algorithm with 
            /// arithmetic shifts and subtractions. 
            ///
            /// ### Formal Definition
            /// For $a, b \in \mathbb{N}_0$, let $D = \{ d \in \mathbb{N} : d \mid a \land d \mid b \}$ 
            /// be the set of common divisors. Then:
            /// $$\gcd(a, b) = \begin{cases} \max(D) & \text{if } (a, b) \neq (0, 0) \\ 0 & \text{if } (a, b) = (0, 0) \end{cases}$$
            ///
            /// ### Complexity
            /// * **Time**: $O(n^2)$ bit operations, where $n$ is the number of bits in the operands.
            /// * **Space**: $O(1)$ auxiliary space.
            /// * **Performance**: Optimized for hardware architectures where bit-shifting (`>>`, `<<`) 
            ///   and trailing zero counting (`ctz`) are faster than integer division.
            ///
            /// ### Properties
            /// * **Commutativity**: $\gcd(a, b) = \gcd(b, a)$
            /// * **Identity**: $\gcd(a, 0) = a$
            /// * **Idempotence**: $\gcd(a, a) = a$
            /// * **Constancy**: This is a `const` function and can be evaluated at compile-time.
            pub const fn gcd(a: $t, b: $t) -> $t {
                stein_iterative(a, b)
            }

            pub const fn gcd_with_strategy(a: $t, b: $t, strategy: GcdStrategy) -> $t {
                match strategy {
                    SteinIterative => stein_iterative(a, b),
                    SteinRecursive => stein_recursive(a, b),
                    EuclideanIterative => euclidean_iterative(a, b),
                    EuclideanSubtraction => euclidean_subtraction(a, b),
                    EuclideanRecursive => euclidean_recursive(a, b),
                }
            }

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

            pub(crate) const fn euclidean_iterative(mut a: $t, mut b: $t) -> $t {
                while b != 0 {
                    if let Some(rem) = a.checked_rem(b) {
                        a = b;
                        b = rem;
                    }
                }
                a
            }

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
                        for (name, func) in FUNCTIONS {
                            std::println!(
                                "Testing {name} implementation for the inputs {a} and {b}."
                            );
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

impl_unsigned_gcd!(u8, gcd_u8);
impl_unsigned_gcd!(u16, gcd_u16);
impl_unsigned_gcd!(u32, gcd_u32);
impl_unsigned_gcd!(u64, gcd_u64);
impl_unsigned_gcd!(u128, gcd_u128);
impl_unsigned_gcd!(usize, gcd_usize);
