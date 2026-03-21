trait GCDAlgorithm {
    fn gcd(a: u64, b: u64) -> u64;
}

struct Euclidean;

impl GCDAlgorithm for Euclidean {
    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            a %= b;
            std::mem::swap(&mut a, &mut b);
        }
        a
    }
}

fn main() {
    let a: u64 = 64;
    let b: u64 = 24;
    let g = Euclidean::gcd(a, b);
    println!("{}", g);
}