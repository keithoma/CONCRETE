const fn stein_iterative(mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }

    let a_zeros = a.trailing_zeros();
    a >>= a_zeros;
    let b_zeros = b.trailing_zeros();
    b >>= b_zeros;
    let common_zeros = if a_zeros < b_zeros { a_zeros } else { b_zeros };

    loop {
        if a > b { (a, b) = (b, a); }
        if let Some(diff) = b.checked_sub(a) { b = diff;}
        if b == 0 { break; }
        b >>= b.trailing_zeros();
    }

    a << common_zeros
}


const fn stein_recursive(a: u64, b: u64) -> u64 {
    match (a, b) {
        (0, y) => y,
        (x, 0) => x,
        (x, y) => {
            match (x & 1 == 0, y & 1 == 0) {
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
            }
        }
    }
}

const fn euclidean_iterative(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        if let Some(rem) = a.checked_rem(b) {
            a = b;
            b = rem;
        }
    }
    a
}

const fn euclidean_subtraction(mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }

    while a != b {
        if let Some(diff) = a.checked_sub(b) {
            a = diff;
        } else if let Some(diff) = b.checked_sub(a) {
            b = diff;
        } else {
            break; // this should never be reached
        }
    }
    a
}

const fn euclidean_recursive(a: u64, b: u64) -> u64 {
    if b != 0 && let Some(rem) = a.checked_rem(b) {
        euclidean_recursive(b, rem)
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
                    result, 
                    expected, 
                    "{name} failed for gcd({a}, {b}). \
                    Expected {expected}, got {result}"
                );
            }
        }
    }
}