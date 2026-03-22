#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]

/// Mathematical operations related to digits.
pub trait Digits {
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
}

impl Digits for u64 {
    fn digit_length(self) -> u32 {
        self.digit_length_logarithmic()
    }

    fn digit_length_iterative(self) -> u32 {
        if self == 0 { return 1 }

        let mut n: u64 = self;
        let mut i: u32 = 0;
        while n > 0 {
            n /= 10;
            i += 1;
        }

        i
    }

    fn digit_length_logarithmic(self) -> u32 {
        // ilog10() panics if the number is 0.
        if self == 0 { return 1; }
        self.ilog10() + 1
    }

    fn get(self, i: u32) -> Option<u8> {
        if i >= self.digit_length() { return None; }
        Some(((self / 10_u64.pow(i)) % 10) as u8)
    }

    fn digit_sum(self) -> u32 {
        let mut n: u64 = self;
        let mut result: u64 = 0;
        while n > 0 {
            result += n % 10;
            n /= 10;
        }
        result as u32
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    // This imports the Digits trait and anything else from the main file 
    // into this isolated testing module.
    use super::*; 

    #[test]
    fn test_iterative_length() {
        let n: u64 = 12345;
        assert_eq!(n.digit_length_iterative(), 5);
        
        let zero: u64 = 0;
        assert_eq!(zero.digit_length_iterative(), 1);
    }

    #[test]
    fn test_logarithmic_length() {
        let n: u64 = 987654321;
        assert_eq!(n.digit_length_logarithmic(), 9);

        let zero: u64 = 0;
        assert_eq!(zero.digit_length_logarithmic(), 1);
    }

    #[test]
    fn test_get() {
        let n: u64 = 123456789;
        assert_eq!(n.get(1), Some(8));
        assert_eq!(n.get(8), Some(1));
        assert_eq!(n.get(9), None);
    }

    #[test]
    fn test_digit_sum() {
        let n: u64 = 12345;
        assert_eq!(n.digit_sum(), 15);
    }
}