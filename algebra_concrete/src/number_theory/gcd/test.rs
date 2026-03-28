macro_rules! test_unsigned_gcd {
    ($t:ty, $mod_name:ident) => {
        #[cfg(test)]
        mod $mod_name {
            use super::super::$mod_name as implementation;
            use super::super::GcdStrategy;

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
                ("gcd", implementation::gcd),
                ("iterative Stein's algorithm", implementation::stein_iterative),
                ("recursive Stein's algorithm", implementation::stein_recursive),
                ("Iterative", implementation::euclidean_iterative),
                ("Subtraction", implementation::euclidean_subtraction),
                ("Recursive", implementation::euclidean_recursive),
            ];

            #[test]
            fn test_all() {
                for (a, b, expected) in CASES {
                    assert_eq!(
                        implementation::gcd_with_strategy(a, b, GcdStrategy::EuclideanRecursive), 
                        expected
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


macro_rules! test_signed_gcd {
    ($t_signed:ty, $t_unsigned:ty, $signed_mod:ident, $unsigned_mod:ident) => {
        #[cfg(test)]
        mod $signed_mod {
            use super::super::$signed_mod as implementation;
            use super::super::GcdStrategy;

            const MAX: $t_signed = <$t_signed>::MAX;
            const MAX_UNSINGED: $t_unsigned = <$t_unsigned>::MAX;
            const CASES: [($t_signed, $t_signed, $t_unsigned); 2] = [
                (-1, 0, 1),
                (-17, -7, 1),
            ];

            type FuncDef = fn($t_signed, $t_signed) -> $t_unsigned;
            const FUNCTIONS: &[(&str, FuncDef)] = &[
                ("gcd", implementation::gcd),
                ("iterative Stein's algorithm", implementation::stein_iterative),
                ("recursive Stein's algorithm", implementation::stein_recursive),
                ("Iterative", implementation::euclidean_iterative),
                ("Subtraction", implementation::euclidean_subtraction),
                ("Recursive", implementation::euclidean_recursive),
            ];

            #[test]
            fn test_all() {
                for (a, b, expected) in CASES {
                    assert_eq!(
                        implementation::gcd_with_strategy(a, b, GcdStrategy::EuclideanRecursive), 
                        expected
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

// Invoke the test generators
test_unsigned_gcd!(u8, gcd_u8);
test_unsigned_gcd!(u32, gcd_u32);
test_unsigned_gcd!(u64, gcd_u64);

test_signed_gcd!(i8, u8, gcd_i8, gcd_u16);
test_signed_gcd!(i32, u32, gcd_i32, get_u64);
test_signed_gcd!(i64, u64, gcd_i64, get_u128);