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

    fn to_u64(&self) -> u64 {
        let mut u: u64 = 0;
        let mut i: u32 = self.len() as u32;
        for &d in &self.digits {
            u += (d as u64) * 10u64.pow(i - 1);
            i -= 1;
        }
        u
    }

    /// Checks if the digits are empty or not.
    fn is_empty(&self) -> bool {
        self.digits.is_empty()
    }

    /// Returns the length of the `Digits` instance.
    fn len(&self) -> usize {
        self.digits.len()
    }

    /// Returns the value at the index if it exists otherwise she returns None.
    fn get(&self, i: usize) -> Option<u8> {
        self.digits.get(i).copied()
    }

    fn first(&self) -> Option<u8> {
        self.get(0)
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

    /// Returns the alternating sum.
    fn alternating_sum(&self) -> i32 {
        let mut result: i32 = 0;
        let mut sign: bool = true;
        for &d in self.digits.iter().rev() {
            result += (2 * (sign as i32) - 1) * (d as i32);
            sign = if sign { false } else { true };
        }
        result
    }

    fn reverse(&mut self) {
        self.digits.reverse();
    }

}

fn main() {
    let mut d = Digits::from_u64(1234);
    let mut n = Digits::new();
    println!("{:?}", d.to_u64());
    println!("Alternating sum: {:?}", d.alternating_sum());
    println!("{:?}", d.is_empty());
    println!("{:?}", n.is_empty());
    println!("{:?}", d.digits);
    println!("{:?}", d.len());
    println!("{:?}", d.get(3));
    println!("{:?}", d.sum());
    println!("{:?}", d.first());
    println!("{:?}", d.last());
    d.reverse();
    println!("{:?}", d.digits);
}