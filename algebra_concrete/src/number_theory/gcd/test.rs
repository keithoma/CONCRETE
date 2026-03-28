macro_rules! test_unsigned_gcd {
    ($t:ty, $mod_name:ident) => {
        #[cfg(test)]
        mod $mod_name {
            use super::super::$mod_name as implementation;
            use super::super::GcdStrategy;

            const MAX: $t = <$t>::MAX;
            
            // Expanded edge cases for comprehensive testing
            const CASES: [($t, $t, $t); 13] = [
                // Zero and Identity cases
                (0, 0, 0),
                (0, MAX, MAX),
                (MAX, 0, MAX),
                (1, MAX, 1),
                (MAX, 1, 1),

                // Max cases
                (MAX, MAX, MAX),

                // Standard coprime and common divisors
                (48, 18, 6),
                (18, 48, 6), // Commutativity check
                (17, 7, 1),

                // Identical numbers
                (42, 42, 42),

                // Multiples
                (15, 45, 15),
                (45, 15, 15),

                // Powers of 2 (Crucial for Stein's algorithm bit-shifting logic)
                (32, 24, 8),
            ];

            type FuncDef = fn($t, $t) -> $t;
            const FUNCTIONS: &[(&str, FuncDef)] = &[
                ("gcd", implementation::gcd),
                ("iterative Stein's algorithm", implementation::stein_iterative),
                ("recursive Stein's algorithm", implementation::stein_recursive),
                ("Iterative", implementation::euclidean_iterative),
                ("Subtraction", implementation::euclidean_subtraction),
                ("Recursive", implementation::euclidean_recursive),
            ];

            const STRATEGIES: &[GcdStrategy] = &[
                GcdStrategy::SteinIterative,
                GcdStrategy::SteinRecursive,
                GcdStrategy::EuclideanIterative,
                GcdStrategy::EuclideanRecursive,
                GcdStrategy::EuclideanSubtraction,
            ];

            #[test]
            fn test_all() {
                for (a, b, expected) in CASES {
                    // 1. Test the strategy router exhaustively (No longer hardcoded!)
                    for &strategy in STRATEGIES {
                        let result = implementation::gcd_with_strategy(a, b, strategy);
                        assert_eq!(
                            result, expected,
                            "{:?} strategy failed for gcd({a}, {b}). Expected {expected}, got {result}",
                            strategy
                        );
                    }

                    // 2. Test the raw internal functions
                    for (name, func) in FUNCTIONS {
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

macro_rules! test_signed_gcd {
    ($t_signed:ty, $t_unsigned:ty, $signed_mod:ident, $unsigned_mod:ident) => {
        #[cfg(test)]
        mod $signed_mod {
            use super::super::$signed_mod as implementation;
            use super::super::GcdStrategy;

            const MAX: $t_signed = <$t_signed>::MAX;
            const MIN: $t_signed = <$t_signed>::MIN;
            
            // Exhaustive edge cases for signed integers
            const CASES: [($t_signed, $t_signed, $t_unsigned); 11] = [
                // Zero cases
                (0, 0, 0),
                (-1, 0, 1),
                (0, -1, 1),
                
                // Extremes (The Two's Complement traps)
                (MAX, MAX, MAX as $t_unsigned),
                (MIN, MIN, MIN.unsigned_abs()),
                (MIN, 0, MIN.unsigned_abs()),
                (0, MIN, MIN.unsigned_abs()),
                (MIN, MAX, 1), // Usually coprime

                // Mixed signs and standard math
                (-48, 18, 6),
                (48, -18, 6),
                (-48, -18, 6),
            ];

            type FuncDef = fn($t_signed, $t_signed) -> $t_unsigned;
            const FUNCTIONS: &[(&str, FuncDef)] = &[
                ("gcd", implementation::gcd),
            ];

            const STRATEGIES: &[GcdStrategy] = &[
                GcdStrategy::SteinIterative,
                GcdStrategy::SteinRecursive,
                GcdStrategy::EuclideanIterative,
                GcdStrategy::EuclideanRecursive,
                GcdStrategy::EuclideanSubtraction,
            ];

            #[test]
            fn test_all() {
                for (a, b, expected) in CASES {
                    // 1. Test the strategy router exhaustively
                    for &strategy in STRATEGIES {
                        let result = implementation::gcd_with_strategy(a, b, strategy);
                        assert_eq!(
                            result, expected,
                            "{:?} strategy failed for gcd({a}, {b}). Expected {expected}, got {result}",
                            strategy
                        );
                    }

                    // 2. Test the raw internal functions
                    for (name, func) in FUNCTIONS {
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

test_unsigned_gcd!(u8, gcd_u8);
test_unsigned_gcd!(u16, gcd_u16);
test_unsigned_gcd!(u32, gcd_u32);
test_unsigned_gcd!(u64, gcd_u64);
test_unsigned_gcd!(u128, gcd_u128);
test_unsigned_gcd!(usize, gcd_usize);

test_signed_gcd!(i8, u8, gcd_i8, gcd_u8);
test_signed_gcd!(i16, u16, gcd_i16, gcd_u16);
test_signed_gcd!(i32, u32, gcd_i32, gcd_u32);
test_signed_gcd!(i64, u64, gcd_i64, gcd_u64);
test_signed_gcd!(i128, u128, gcd_i128, gcd_u128);