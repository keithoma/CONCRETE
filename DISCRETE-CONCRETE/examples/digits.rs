/*
=============================================================================
 CODE REVIEW & IMPROVEMENT NOTES FOR `Digits`
=============================================================================

1. OPTIMIZATIONS & BUG FIXES:
   - `from_u64`: Avoid `digits_vector.insert(0, ...)`. It shifts the entire 
     array every time (O(n^2)). Instead, use `push(...)` and then call 
     `digits_vector.reverse()` after the loop.
   - `to_u64`: Avoid `10u64.pow()`. Use Horner's method to accumulate the 
     number: `let mut u = 0; for &d in &self.digits { u = u * 10 + (d as u64); }`
   - `last`: `self.get(self.len() - 1)` will panic (underflow) if the vector 
     is empty. Use the native `self.digits.last().copied()` instead.
   - `first`: Can be simplified to `self.digits.first().copied()`.
   - `sum`: Make it idiomatic with iterators: 
     `self.digits.iter().map(|&d| d as u32).sum()`

2. IDIOMATIC RUST TRAITS TO IMPLEMENT:
   - `Default`: Instead of just `new()`, implement the `Default` trait.
   - `From<u64>`: Replace `from_u64` by implementing `From<u64> for Digits`.
     (Usage: `Digits::from(1234)`)
   - `From<&Digits> for u64`: Replace `to_u64` by implementing this trait. 
   - `std::fmt::Display`: Implement this so you can `println!("{}", d)` 
     instead of needing `{:?}`.

3. NEW FEATURES TO ADD (For Divisibility):
   - `is_divisible_by_3(&self) -> bool` (using `self.sum() % 3 == 0`)
   - `is_divisible_by_11(&self) -> bool` (using `self.alternating_sum() % 11 == 0`)
   - `FromStr`: Implement `std::str::FromStr` to construct `Digits` from a String.
=============================================================================
*/

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
    }1

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
            sign = !sign;
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