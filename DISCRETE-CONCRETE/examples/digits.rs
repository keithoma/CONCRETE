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
        let mut digits_vector = Vec::new();
        while n > 0 {
            digits_vector.insert(0, (n % 10) as u8);
            n /= 10;
        }
        Self {
            digits: digits_vector
        }
    }

    /// Returns the length of the `Digits` instance.
    fn len(&self) -> usize {
        self.digits.len()
    }

    /// Returns the value at the index if it exists otherwise she returns None.
    fn get(&self, i: usize) -> Option<u8> {
        self.digits.get(i).copied()
    }

    /// Returns the last digit.
    fn last(&self) -> Option<u8> {
        self.get(self.len() -1)
    }

    /// Returns the digit sum.
    fn sum(&self) -> u32 {
        let mut result: u32 = 0;
        for d in &self.digits {
            result += *d as u32;
        }
        result
    }

}

fn main() {
    let d = Digits::from_u64(123456);
    println!("{:?}", d.digits);
    println!("{:?}", d.len());
    println!("{:?}", d.get(3));
    println!("{:?}", d.sum());
    println!("{:?}", d.last());
}