trait Primality {
    /// Returns `true` if `n` is prime and `false` otherwise.
    fn is_prime(n: u64) -> bool;
}

/// Primality test by trivial division.
struct TrivialDivision;

impl Primality for TrivialDivision {
    fn is_prime(n: u64) -> bool {
        // 0 and 1 are not primes by convention.
        if n <= 1 {
            return false;
        }

        for i in 2..n {
            // We only need to check divisibility to the square root of n
            // rounded down. We are also avoiding float numbers here.
            if i > n / i {
                break;
            }

            if n % i == 0 {
                return false;
            }
        }
        true
    }
}


trait PrimeFactorizationAlgorithm {
    fn prime_factorization(a: u64) -> Result<Vec<(u64, u32)>, String>;
}

trait GCDAlgorithm {
    fn gcd(a: u64, b: u64) -> u64;
}

struct Euclidean;

impl GCDAlgorithm for Euclidean {
    fn gcd(mut a: u64, mut b: u64) -> u64 {
        a
    }
}

fn main() {
    let a: u64 = 64;
    println!("{}", TrivialDivision::is_prime(a));
}