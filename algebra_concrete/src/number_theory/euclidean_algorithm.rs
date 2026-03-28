const fn euclidean_algorithm_iterative(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        if let Some(rem) = a.checked_rem(b) {
            a = b;
            b = rem;
        } else {
            return a;
        }
    }
    a
}

const fn euclidean_algorithm_subtraction(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    }

    if b == 0 {
        return a;
    }

    while a != b {
        if a > b && let Some(diff) = a.checked_sub(b) {
            a = diff;
        } else if a < b && let Some(diff) = b.checked_sub(a) {
            b = diff;
        } else {
            break;
        }
    }
    a
}

const fn euclidean_algorithm_recursive(a: u64, b: u64) -> u64 {
    if b != 0 && let Some(rem) = a.checked_rem(b) {
        euclidean_algorithm_recursive(b, rem)
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
                    "Implementation failed for inputs (`a`, `b`)"
                );
            }
        }
    }
}