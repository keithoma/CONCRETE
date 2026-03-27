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
    /// Returns an [`DigitIter`] iterator over the digits of the number from
    /// most-significant to least-significant.
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

    /// Returns the number of digits in the integer in base 10.
    ///
    /// This method treats `0` as having a length of `1`. For any other 
    /// positive integer, it returns the number of decimal digits required 
    /// to represent it.
    ///
    /// # Performance
    ///
    /// This operation is $O(1)$. It uses a hardware-accelerated integer 
    /// logarithm (`ilog10`) to determine the length without iteration.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate_name::Digits;
    ///
    /// assert_eq!(0u64.digit_length(), 1);
    /// assert_eq!(7u64.digit_length(), 1);
    /// assert_eq!(123u64.digit_length(), 3);
    /// assert_eq!(u64::MAX.digit_length(), 20);
    /// ```
    fn digit_length(self) -> usize;

    /// Returns the digit at the given power of 10 index.
    ///
    /// This method treats the integer as a sequence of digits where index `0` 
    /// corresponds to the ones place ($10^0$), index `1` to the tens place ($10^1$), 
    /// and so on.
    ///
    /// Returns `None` if the index `i` is greater than or equal to the 
    /// number of digits.
    ///
    /// # Performance
    ///
    /// This operation is $O(1)$. It performs a constant-time array lookup 
    /// without re-calculating powers or performing division.
    ///
    /// # Examples
    ///
    /// ```
    /// use discrete::Digits;
    ///
    /// let n = 12345u64;
    /// assert_eq!(n.get(0), Some(5)); // Ones place (10^0)
    /// assert_eq!(n.get(1), Some(4)); // Tens place (10^1)
    /// assert_eq!(n.get(4), Some(1)); // Ten-thousands place (10^4)
    /// assert_eq!(n.get(5), None);    // Out of bounds
    ///
    /// // Zero is the only number with a digit at index 0 that is 0.
    /// assert_eq!(0u64.get(0), Some(0));
    /// assert_eq!(0u64.get(1), None);
    ///
    /// // u64::MAX is 18,446,744,073,709,551,615
    /// assert_eq!(u64::MAX.get(0), Some(5));  // The last '5'
    /// assert_eq!(u64::MAX.get(19), Some(1)); // The leading '1'
    /// assert_eq!(u64::MAX.get(20), None);
    /// ```
    #[inline]
    fn get(self, i: usize) -> Option<u8>;

    /// Returns the sum of the digits in base 10.
    ///
    /// # Performance
    ///
    /// This operation is $O(n)$, where $n$ is the number of digits.
    ///
    /// # Examples
    ///
    /// ```
    /// use discrete::Digits;
    ///
    /// assert_eq!(0u64.digit_sum(), 0);
    /// assert_eq!(12345u64.digit_sum(), 15);
    /// assert_eq!(u64::MAX.digit_sum(), 91);
    /// ```
    #[inline]
    fn digit_sum(self) -> u8;
    
    /// Returns the alternating digit sum in base 10.
    ///
    /// The sum is calculated starting from the least-significant digit (index 0) 
    /// with a positive sign, then subtracting the next digit, and so on: 
    /// $d_0 - d_1 + d_2 - d_3 \dots$
    ///
    /// # Performance
    ///
    /// This operation is $O(n)$, where $n$ is the number of digits.
    ///
    /// # Examples
    ///
    /// ```
    /// use discrete::Digits;
    ///
    /// // 123 -> 3 - 2 + 1 = 2
    /// assert_eq!(123u64.alternating_digit_sum(), 2);
    ///
    /// // 1234 -> 4 - 3 + 2 - 1 = 2
    /// assert_eq!(1234u64.alternating_digit_sum(), 2);
    ///
    /// // 0 -> 0
    /// assert_eq!(0u64.alternating_digit_sum(), 0);
    ///
    /// // For u64::MAX, the alternating sum is -7.
    /// assert_eq!(u64::MAX.alternating_digit_sum(), -7);
    /// ```
    #[inline]
    fn alternating_digit_sum(self) -> i8;

    /// Returns the integer formed by reversing the decimal digits.
    ///
    /// The reversal is performed in base 10. Note that leading zeros in the 
    /// reversed result are omitted (e.g., `100` reversed is `1`).
    ///
    /// # Panics
    ///
    /// This method will panic in debug builds if the reversed value exceeds 
    /// [`u64::MAX`]. In release builds, it will perform two's complement wrapping.
    ///
    /// # Performance
    ///
    /// This operation is $O(n)$, where $n$ is the number of digits. It uses
    /// a single-pass accumulation (Horner's method) over the [`DigitIter`].
    ///
    /// # Examples
    ///
    /// ```
    /// use discrete::Digits;
    ///
    /// assert_eq!(123u64.reverse(), 321);
    /// assert_eq!(100u64.reverse(), 1);
    /// assert_eq!(0u64.reverse(), 0);
    /// ```
    #[inline]
    fn reverse(self) -> u64;


    /// Checks if the number is a palindrome in base 10.
    ///
    /// A palindrome is a number that remains the same when its digits are reversed.
    /// Single-digit numbers are always palindromes.
    ///
    /// # Performance
    ///
    /// This operation is $O(n)$ in the worst case, where $n$ is the number of digits.
    /// It uses a double-ended iterator to compare digits from both ends 
    /// simultaneously, exiting as soon as a mismatch is found.
    ///
    /// # Examples
    ///
    /// ```
    /// use discrete::Digits;
    ///
    /// assert!(123321u64.is_palindrome());
    /// assert!(1234321u64.is_palindrome());
    /// assert!(0u64.is_palindrome());
    /// assert!(!10u64.is_palindrome());
    /// assert!(!u64::MAX.is_palindrome());
    /// ```
    #[inline]
    fn is_palindrome(self) -> bool;


    ///
    fn is_narcissistic(self) -> bool;


    /// Returns the digital root of the number in base 10.
    ///
    /// The digital root is the value obtained by an iterative process of summing digits
    /// until a single-digit number is reached. For example, the digital root of 12,345 is 6,
    /// because $1+2+3+4+5 = 15$, and $1+5 = 6$.
    ///
    /// # Performance
    ///
    /// This operation is $O(1)$. It uses the constant-time modulo 9 congruence formula.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate_name::Digits;
    /// assert_eq!(12345u64.digital_root(), 6);
    /// ```
    fn digital_root(self) -> u8;

    /// Returns the digital root using the congruence formula: $1 + ((n - 1) \pmod 9)$.
    ///
    /// This is the $O(1)$ implementation used by [`Self::digital_root`].
    fn digital_root_modulo(self) -> u8;

    /// Returns the digital root by actually performing the iterative summation.
    ///
    /// Unlike [`Self::digital_root_modulo`], this method follows the literal 
    /// definition of summing digits recursively until a single digit remains.
    ///
    /// # Performance
    ///
    /// This operation is $O(n^2)$ in the worst case (where $n$ is the number of digits).
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