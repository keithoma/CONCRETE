
fn steins_algorithm(mut a: u64, mut b: u64) -> u64 {
    if b == 0 {
        a
    } else if a % 2 == 0 && b % 2 == 0 {
        steins_algorithm(a / 2, b / 2)
    } else if a % 2 != 0 && b % 2 == 0 {
        steins_algorithm(a, b / 2)
    } else {
        steins_algorithm(a, b)
    }
}

const fn euclidean_algorithm_iterative(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        if let Some(rem) = a.checked_rem(b) {
            a = b;
            b = rem;
        } else {
            return a; // should never be reached
        }
    }
    a
}

const fn euclidean_algorithm_subtraction(mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }

    while a != b {
        if a > b {
            a = match a.checked_sub(b) {
                Some(diff) => diff,
                None => break, // should never be reached
            }
        } else {
            b = match b.checked_sub(a) {
                Some(diff) => diff,
                None => break, // should never be reached
            }
        }
    }
    a
}

const fn euclidean_algorithm_recursive(a: u64, b: u64) -> u64 {
    if b != 0 {
        match a.checked_rem(b) {
            Some(rem) => euclidean_algorithm_recursive(b, rem),
            None => a, // should never be reached
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