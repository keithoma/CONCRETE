/*
=============================================================================
 CODE REVIEW & IMPROVEMENT NOTES FOR `Digits`
=============================================================================

1. OPTIMIZATIONS & BUG FIXES:
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
   - `FromStr`: Implement `std::str::FromStr` to construct `Digits` from a String.
=============================================================================
*/

/*
=============================================================================
 MODULE OBJECTIVES: `Digits` 
 Status: IN PROGRESS
=============================================================================
SCOPE: 
This module is strictly a foundational data structure for representing 
base-10 digits. It handles bidirectional conversion and basic sequence 
operations. 

RULE: NO business logic or divisibility checks belong in this file.

THE "DEFINITION OF DONE" CHECKLIST:
[ ] 1. SAFETY & SPEED: 

[ ] 2. IDIOMATIC TRAITS:
       - `impl Default for Digits`
       - `impl From<u64> for Digits`
       - `impl From<&Digits> for u64`
       - `impl std::fmt::Display for Digits`

[ ] 3. ITERATION:
       - `impl IntoIterator for &Digits`

[ ] 4. INNATE PROPERTIES:
       - `sum() -> u32`
       - `alternating_sum() -> i32`
       - `reverse()`

Once these boxes are checked, this file is COMPLETE. 
=============================================================================
*/

/*
=============================================================================
 MODULE OBJECTIVES: `Digits` (Number Theory & Combinatorics)
=============================================================================
THE "REASON TO EXIST":
Standard Rust integers represent scalar values, making it computationally 
expensive to analyze their structural properties. This struct treats numbers 
as sequences, enabling fast combinatorial and number-theoretic analysis.

THE FINAL CHECKLIST:
[ ] 1. SAFETY, SPEED & TRAITS (The Basics): 

       - Implement `Default`, `From<u64>`, `From<&Digits> for u64`.
       - Implement `std::fmt::Display`.

[ ] 2. RECREATIONAL MATH PROPERTIES:


[ ] 3. COMBINATORICS & FREQUENCY:
       - `frequency_map(&self) -> [u8; 10]`
       - `is_anagram_of(&self, other: &Digits) -> bool`
       - `next_permutation(&mut self) -> bool` 
         (Finds the next largest sequence using the same digits)

Once this is done, you have a highly specialized structural analysis tool.
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
            digits_vector.push((n % 10) as u8);
            n /= 10;
        }
        digits_vector.reverse();
        Self {
            digits: digits_vector
        }
    }

    fn from_str(s: &str) -> Self {
        Self {
            digits: s.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u8)
                .collect()
        }
    }

    fn to_u64(&self) -> u64 {
        let mut result: u64 = 0;
        for &digit in &self.digits {
            result = result * 10 + (digit as u64);
        }
        result
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

    /// Returns the first digit.
    fn first(&self) -> Option<u8> {
        self.digits.first().copied()
    }

    /// Returns the last digit.
    fn last(&self) -> Option<u8> {
        self.digits.last().copied()
    }

    /// Returns the digit sum.
    fn sum(&self) -> u64 {
        self.digits.iter().map(|&d| d as u64).sum()
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

    /// Reverses the digits in place.
    fn reverse(&mut self) {
        self.digits.reverse();
    }

    /// Checks if the digits are a palindrome.
    fn is_palindrome(&self) -> bool {
        self.digits.iter()
            .take(self.len() / 2)
            .zip(self.digits.iter().rev())
            .all(|(x, y)| x == y)
    }

    // Computes the digital root using the modular trick.
    fn digital_root(&self) -> u8 {
        let s = self.sum();
        if s == 0 { 0 } else { (1 + (s - 1) % 9) as u8 }
    }

    // Computes the digital root recursively.
    fn digital_root_recursive(&self) -> u8 {
        let result: u64 = self.sum();
        if result < 10 {
            result as u8
        } else {
            Self::from_u64(result).digital_root()
        }
    }

    fn is_narcissistic(&self) -> bool {
        let exponent: u32 = self.len() as u32;
        self.to_u64() == self.digits.iter().map(|&d| (d as u64).pow(exponent)).sum()
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

    let mut d2 = Digits::from_u64(123454321);
    println!("{:?}", d2.is_palindrome());
    println!("{:?}", d2.digital_root());

    let mut d3 = Digits::from_u64(548834);
    println!("is narcisstic: {:?}", d3.is_narcissistic());

    let mut d4 = Digits::from_str("2132131");
    println!("{:?}", d4);

    let mut d5 = Digits::from_str("adsa\nasdasda2132131");
    println!("{:?}", d5);
}