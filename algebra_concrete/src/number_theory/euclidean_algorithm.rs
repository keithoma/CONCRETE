const fn euclidean_algorithm_iterative(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        if let Some(rem) = a.checked_rem(b) {
            a = b;
            b = rem;
        } else {
            return a; // this should be an error
        }
    }
    a
}

const fn euclidean_algorithm_subtraction(mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }

    while a != b {
        if a > b {
            match a.checked_sub(b) {
                Some(diff) => a = diff,
                None => break, // this should be an error
            }
        } else {
            match b.checked_sub(a) {
                Some(diff) => b = diff,
                None => break, // this should be an error
            }
        }
    }
    a
}

const fn euclidean_algorithm_recursive(a: u64, b: u64) -> u64 {
    if b != 0 {
        match a.checked_rem(b) {
            Some(rem) => euclidean_algorithm_recursive(b, rem),
            None => a, // this should be an error
        }
    } else {
        a
    }
}


#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;

    const MAX: u64 = u64::MAX;
    const CASES: [(u64, u64, u64); 7] = [
        (0, 0, 0),
        (0, MAX, MAX),
        (MAX, 0, MAX),
        (MAX, MAX, MAX),
        (48, 18, 6),
        (1071, 462, 21),
        (17, 7, 1),
    ];

    type FuncDef = fn(u64, u64) -> u64;
    const FUNCTIONS: &[(&str, FuncDef)] = &[
        ("Iterative", euclidean_algorithm_iterative),
        ("Subtraction", euclidean_algorithm_subtraction),
        ("Recursive", euclidean_algorithm_recursive),
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
                    result, 
                    expected, 
                    "{name} failed for gcd({a}, {b}). \
                    Expected {expected}, got {result}"
                );
            }
        }
    }
}