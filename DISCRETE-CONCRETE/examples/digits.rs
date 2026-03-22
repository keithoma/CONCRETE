/// Represents the digits of a u64-integer.
#[derive(Debug)]
struct Digits {
    digits: Vec<u8>,
}

impl Digits {
    /// Constructs a new empty Digits instance.
    fn new() -> Self {
        Self {
            digits: Vec::new(),
        }
    }

    /// Constructs and populates new Digits instance with the digits from `n`.
    fn from_u64(n: u64) -> Self {
        if n == 0 {
            return Self {
                digits: vec![0]
            }
        };

        let mut n = n;
        let mut digits = Vec::new();
        while n > 0 {
            digits.insert(0, (n % 10) as u8);
            n /= 10;
        }
        Self {
            digits: digits
        }
    }
}

fn main() {
    let d = Digits::from_u64(123456);
    println!("{:?}", d.digits);
}