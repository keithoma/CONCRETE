#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]

/// An iterator that yields digits from right to left (10^0, 10^1, ...).
pub struct DigitIter {
    number: u64,
}

impl Iterator for DigitIter {
    type Item = u8; // The type we yield

    fn next(&mut self) -> Option<Self::Item> {
        if self.number == 0 {
            None // Stop iterating
        } else {
            let digit = (self.number % 10) as u8;
            self.number /= 10;
            Some(digit)
        }
    }
}

/// Mathematical operations related to digits.
pub trait Digits {
    ///
    fn digits(self) -> DigitIter;

    /// Default function to count the number of digits.
    fn digit_length(self) -> u32;

    /// Counts the number of digits.
    fn digit_length_iterative(self) -> u32;

    /// Counts the number of digits.
    fn digit_length_logarithmic(self) -> u32;

    /// Returns the digit at the index.
    fn get(self, i: u32) -> Option<u8>;

    /// Computes the digit sum.
    fn digit_sum(self) -> u32;
    

    /// Computes the alternating digit sum.
    fn alternating_digit_sum(self) -> i32;

    /* 
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
     */
}

impl Digits for u64 {
    fn digits(self) -> DigitIter {
        DigitIter { number: self }
    }
    
    fn digit_length(self) -> u32 {
        self.digit_length_logarithmic()
    }

    fn digit_length_iterative(mut self) -> u32 {
        if self == 0 { return 1 }
        self.digits().count() as u32
    }

    fn digit_length_logarithmic(self) -> u32 {
        // ilog10() panics if the number is 0.
        if self == 0 { return 1; }
        self.ilog10() + 1
    }

    fn get(self, i: u32) -> Option<u8> {
        // Attempt to calculate 10^i. If it overflows, the index is definitely too high.
        let divisor = 10_u64.checked_pow(i)?;
        
        // If the number is smaller than the divisor, the index is out of bounds.
        if self < divisor {
            return None;
        }

        Some(((self / divisor) % 10) as u8)
    }

    fn digit_sum(mut self) -> u32 {
        self.digits().map(|x| x as u32).sum()
    }

    fn alternating_digit_sum(self) -> i32 {
        self.digits()
            .enumerate()
            .map(|(i, val)| {
                let sign = if i % 2 == 0 { 1 } else { -1 };
                (val as i32) * sign
            })
            .sum()
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
    const LONG_DIGITS: u64 = 123456789;
    const SHORT_DIGITS: u64 = 321;

    #[test]
    fn test_digititer_map() {
        let actual: Vec<u8> = LONG_DIGITS
            .digits()
            .map(|x| x + 1)
            .collect();
        let expected = vec![10, 9, 8, 7, 6, 5, 4, 3, 2];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_digititer_filter() {
        let actual: Vec<u8> = LONG_DIGITS
            .digits()
            .filter(|&x| x >= 8)
            .collect();
        let expected = vec![9, 8];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_digititer_enumerate() {
        let actual: Vec<(usize, u8)> = SHORT_DIGITS
            .digits()
            .enumerate()
            .collect();
        let expected = vec![(0, 1), (1, 2), (2, 3)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_iterative_length() {
        let actual: u32 = ZERO.digit_length_iterative();
        let expected: u32 = 1;
        assert_eq!(actual, expected);

        let actual: u32 = LONG_DIGITS.digit_length_iterative();
        let expected: u32 = 9;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_logarithmic_length() {
        let actual: u32 = ZERO.digit_length_logarithmic();
        let expected: u32 = 1;
        assert_eq!(actual, expected);

        let actual: u32 = LONG_DIGITS.digit_length_logarithmic();
        let expected: u32 = 9;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get() {
        let actual: Option<u8> = LONG_DIGITS.get(0);
        let expected: Option<u8> = Some(1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_digit_sum() {
        let n: u64 = 12345;
        assert_eq!(n.digit_sum(), 15);
    }

    #[test]
    fn test_alternating_digit_sum() {
        let actual: i32 = SHORT_DIGITS.alternating_digit_sum();
        let expected: i32 = 4;
        assert_eq!(actual, expected);
    }
}