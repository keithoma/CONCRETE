#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![crate_name = "doc"]

pub trait Digits {
    fn digit_length_iterative(&self) -> u64;
}

impl Digits for u64 {
    fn digit_length_iterative(&self) -> u64 {
        if self == 0 { return 1 }

        let mut n: u64 = *self;
        let mut i: u64 = 0;
        while n > 0 {
            n /= 10;
            i += 1;
        }

        i
    }

    fn digit_length_logarithmic(&self) -> u64 {
        // ilog10() panics if the number is 0.
        if self == 0 { return 1; }
        
        self.ilog10() + 1
    }
}

fn main() {

}

#[test]