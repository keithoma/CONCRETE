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
    let a: u64 = 17;
    let b: u64 = 2;
    let n: u64 = b % a;
    println!("{}", n);
}