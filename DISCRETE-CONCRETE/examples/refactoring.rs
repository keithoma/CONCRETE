#![no_std]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]

/// An iterator that yields digits from right to left (10^0, 10^1, ...).
pub struct DigitIter {
    digits: [u8; 20],
    front: usize,
    back: usize,
}

impl DigitIter {
    pub fn new(mut n: u64) -> Self {
        let mut digits = [0u8; 20];

        if n == 0 {
            return Self { digits, front: 19, back: 20}
        }

        let mut count = 0;
        while n > 0 {
            digits[19 - count] = (n % 10) as u8;
            n /= 10;
            count += 1;
        }
        Self { digits, front: 20 - count, back: 20}
    }
}

impl Iterator for DigitIter {
    type Item = u8; // The type we yield

    fn next(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            let digit = self.digits[self.front];
            self.front += 1;
            Some(digit)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for DigitIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            self.back -= 1;
            let digit = self.digits[self.back];
            Some(digit)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for DigitIter {
    fn len(&self) -> usize {
        self.back - self.front
    }
}

/// Mathematical operations related to digits.
pub trait Digits {
    /// Returns an [`[DigitIter]`] iterator over the digits of the number from most-significant to least-significant.
    ///
    /// The iterator yields each digit as a `u8`. For `0`, it yields a single `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate_name::Digits;
    ///
    /// let mut digits = 123u64.digits();
    /// assert_eq!(digits.next(), Some(1));
    /// assert_eq!(digits.next(), Some(2));
    /// assert_eq!(digits.next(), Some(3));
    /// assert_eq!(digits.next(), None);
    /// ```
    ///
    /// Since it is a `DoubleEndedIterator`, you can also go backwards:
    ///
    /// ```
    /// use your_crate_name::Digits;
    ///
    /// let mut digits = 123u64.digits();
    /// assert_eq!(digits.next_back(), Some(3));
    /// ```
    fn digits(self) -> DigitIter;

    ///
    fn digit_length(self) -> usize;

    /// Returns the digit at the index.
    fn get(self, i: usize) -> Option<u8>;

    /// Computes the digit sum.
    fn digit_sum(self) -> u8;
    
    /// Computes the alternating digit sum.
    fn alternating_digit_sum(self) -> i8;

    /// Returns an integer with its digits reversed.
    fn reverse(self) -> u64;


    ///
    fn is_palindrome(self) -> bool;


    ///
    fn is_narcissistic(self) -> bool;


    ///
    fn digital_root(self) -> u8;

    ///
    fn digital_root_modulo(self) -> u8;

    ///
    fn digital_root_recursive(self) -> u8;
}

impl Digits for u64 {
    fn digits(self) -> DigitIter {
        DigitIter::new(self)
    }
    
    fn digit_length(self) -> usize {
        if self == 0 { return 1; }
        (self.ilog10() + 1) as usize
    }

    fn get(self, i: usize) -> Option<u8> {
        let digits = self.digits();
        if i < digits.len() {
            Some(digits.digits[19 - i])
        } else {
            None
        }
    }

    fn digit_sum(self) -> u8 {
        self.digits().sum()
    }

    fn alternating_digit_sum(self) -> i8 {
        self.digits()
            .rev()
            .enumerate()
            .map(|(i, d)| {
                let digit = d as i8;
                if i % 2 == 0 { digit } else { -digit }
            })
            .sum()
    }

    fn reverse(self) -> u64 {
        self.digits().fold(0u64, |acc, d| acc * 10 + d as u64)
    }

    fn is_palindrome(self) -> bool {
        let mut digits = self.digits();
        while let (Some(f), Some(b)) = (digits.next(), digits.next_back()) {
            if f != b { return false; }
        }
        true
    }

    fn is_narcissistic(self) -> bool {
        let digits = self.digits();
        let n = digits.len() as u32;
        digits.map(|x| (x as u64).pow(n)).sum()::<u64> == self
    }

    fn digital_root(self) -> u8 {
        digital_root_modulo(self)
    }

    fn digital_root_modulo(self) -> u8 {
        if self == 0 {
            0
        } else {
            // The formula: 1 + (n - 1) % 9
            (1 + (self - 1) % 9) as u8
        }
    }

    fn digital_root_recursive(self) -> u8 {
        if self < 10 {
            self as u8
        } else {
            (self.digit_sum() as u64).digital_root()
        }
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    // This imports the Digits trait and anything else from the main file 
    // into this isolated testing module.
    use super::*; 

    const ZERO: u64 = 0;
    const SHORT_DIGITS: u64 = 321;
    const LONG_DIGITS: u64 = 123456789;

    #[test]
    fn test_digit_length() {
        let actual: usize = ZERO.digit_length();
        let expected: usize = 1;
        assert_eq!(actual, expected);

        let actual: usize = SHORT_DIGITS.digit_length();
        let expected: usize = 3;
        assert_eq!(actual, expected);

        let actual: usize = LONG_DIGITS.digit_length();
        let expected: usize = 9;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_digit_sum() {
        let actual: u8 = ZERO.digit_sum();
        let expected: u8 = 0;
        assert_eq!(actual, expected);
    }


    // #[test]
    // fn test_alternating_digit_sum() {
    //     let actual: i32 = SHORT_DIGITS.alternating_digit_sum();
    //     let expected: i32 = 4;
    //     assert_eq!(actual, expected);
    // }
}