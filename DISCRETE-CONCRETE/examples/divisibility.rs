// to-do: use turbo fish to get one unifying function
// let check = 369.is_divisible_by::<Three>();

pub mod strategy {
    /// Strategies for checking if a number is divisible by 2.
    #[derive(Default)]
    pub enum By2 {
        /// The last digit is even.
        #[default]
        LastDigitEven,
        /// Standard remainder check (n % 2 == 0).
        ModuloOperator
    }


    /// Strategies for checking if a number is divisible by 3.
    #[derive(Default)]
    pub enum By3 {
        /// The sum of digits is divisible by 3.
        #[default]
        DigitSum,
        /// The difference of the count of digits {1, 4, 7} and the count of
        /// digits {2, 5, 8} is divisible by 3.
        DigitClassCount,
        /// Subtracting twice the last digit from the rest is divisible by 3.
        SubtractDoubleLastDigit,
        /// Standard remainder check (n % 3 == 0).
        ModuloOperator
    }


    /// Strategies for checking if a number is divisible by 4.
    #[derive(Default)]
    pub enum By4 {
        /// The last two digits are divisible by 4.
        #[default]
        LastTwoDigits,
        /// If the tens digit is even, the ones is 0, 4 or 8. Otherwise the
        /// ones are 2 or 6.
        TensDigitOnesDigit,
        /// The sum of the ones digit and double the tens digit is divisible
        /// by 4.
        TwiceTensPlusOnes,
        /// Standard remainder check (n % 4 == 0).
        ModuloOperator
    }


    /// Strategies for checking if a number is divisible by 5.
    #[derive(Default)]
    pub enum By5 {
        /// The last digit is 0 or 5.
        #[default]
        LastDigit,
        /// Standard remainder check (n % 5 == 0).
        ModuloOperator
    }


    /// Strategies for checking if a number is divisible by 6.
    #[derive(Default)]
    pub enum By6 {
        /// Divisible by both 2 and 3.
        #[default]
        TwoAndThree,
        /// The sum of the ones digit, 4 times the tens digit, 4 times the
        /// hundreds digit, 4 times of the thousands digit is divisible by
        /// 6.
        WeightedSumOfDigits,
        /// Standard remainder check (n % 6 == 0).
        ModuloOperator
    }
}



trait DivisibilityStrategy {
    /// Returns `true` if `n` satisfies the strategy's logic.
    fn is_satisfied_by(&self, n: u64) -> bool;
}

trait Divisible {
    fn is_divisible_with(self, method: impl DivisibilityStrategy) -> bool;

    fn is_divisible_by<S: DivisibilityStrategy + Default>(self) -> bool 
    where 
        Self: Sized 
    {
        self.is_divisible_with(S::default())
    }
}



impl DivisibilityStrategy for strategy::By2 {
    fn is_satisfied_by(&self, n: u64) -> bool {
        match self {
            Self::LastDigitEven => matches!(n % 10, 0 | 2 | 4 | 6 | 8),
            Self::ModuloOperator => n % 2 == 0,
        }
    }

}

impl DivisibilityStrategy for strategy::By3 {
    fn is_satisfied_by(&self, n: u64) -> bool {
        match self {
            Self::DigitSum => {
                if n < 10 {
                    crate::digit::three_divides(n)
                } else {
                    n
                    .digit_sum()
                    .is_divisible_with(Self::DigitSum)
                }
            }

            Self::DigitClassCount => {
                let mut remainder = n;
                let mut quantity_1_4_7: u64 = 0;
                let mut quantity_2_5_8: u64 = 0;

                while remainder > 0 {
                    let digit: u64 = remainder % 10;
                    if matches!(digit, 1 | 4 | 7) {
                        quantity_1_4_7 += 1;
                    } else if matches!(digit, 2 | 5 | 8) {
                        quantity_2_5_8 += 1;
                    }
                    remainder /= 10;
                }

                let difference: u64 = if quantity_1_4_7 >= quantity_2_5_8 {
                    quantity_1_4_7 - quantity_2_5_8
                } else {
                    quantity_2_5_8 - quantity_1_4_7
                };

                if difference < 10 {
                    crate::digit::divisible_by_3(difference)
                } else {
                    difference.is_divisible_by(Self::DigitClassCount)
                }
            }

            Self::SubtractDoubleLastDigit => {
                if n < 10 {
                    crate::digit::divisible_by_3(n)
                } else {
                    let new_n: u64 = (n / 10).abs_diff(2 * (n % 10));
                    new_n.is_divisible_by(Self::SubtractDoubleLastDigit)
                }
            }

            Self::ModuloOperator => n % 3 == 0
        }
    }
}

impl DivisibilityStrategy for strategy::By4 {
    fn is_satisfied_by(&self, n: u64) -> bool {
        match self {
            Self::LastTwoDigits => {
                let divisible_last_two_digits: [u64; 25] = [
                    0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48,
                    52, 56, 60, 64, 68, 72, 76, 80, 84, 88, 92, 96
                ];

                let last_two_digits: u64 = n % 100;
                divisible_last_two_digits.contains(&last_two_digits)
            }

            Self::TensDigitOnesDigit => {
                let tens_digit: u64 = (n % 100) / 10;
                let ones_digit: u64 = n % 10;
                if crate::digit::two_divides(tens_digit) {
                    matches!(ones_digit, 0 | 4 | 8)
                } else {
                    matches!(ones_digit, 2 | 6)
                }
            }

            Self::TwiceTensPlusOnes => {
                if n < 10 {
                    crate::digit::four_divides(n)
                } else {
                    let intermediate_result: u64 = 2 * ((n % 100) / 10) + (n % 10);
                    intermediate_result.is_divisible_by(Self::TwiceTensPlusOnes)
                }
            }

            Self::ModuloOperator => {
                n % 4 == 0
            }
        }

    }

}

impl DivisibilityStrategy for strategy::By5 {
    fn is_satisfied_by(&self, n: u64) -> bool {
        match self {
            Self::LastDigit => {
                matches!(n % 10, 0 | 5)
            }

            Self::ModuloOperator => {
                n % 5 == 0
            }
        }
    }
}

impl DivisibilityStrategy for strategy::By6 {
    fn is_satisfied_by(&self, n: u64) -> bool {
        match self {
            Self::TwoAndThree => {
                n.is_divisible_by::<strategy::By2>() &&
                n.is_divisible_by::<strategy::By3>()
            }

            Self::SumOfDigits => {
                let intermediate: u64 = 4 * (
                    n.get_digit(4) + n.get_digit(3) + n.get_digit(2)
                ) + n.get_digit(1);

                if intermediate < 10 {
                    crate::digit::six_divides(intermediate)
                } else {
                    intermediate.is_divisible_by(Self::SumOfDigits)
                }
            }

            Self::ModuloOperator => {
                n % 6 == 0
            }
        }
    }
}


impl Divisible for u64 {
    fn is_divisible_with(self, method: impl DivisibilityStrategy) -> bool {
        method.is_satisfied_by(self)
    }
}

impl u64 {
    /// Returns the number of digits in base-10.
    fn digit_count(self) -> u32 {
        if self == 0 {
            return 1;
        }
        self.ilog10() + 1
    }

    fn get_digit(self, i: u32) -> u64 {
        (self / 10u64.pow(i - 1)) % 10 
    }

    fn to_digits(self) -> Vec<u64> {
        if self == 0 {
            return vec![0];
        }

        let mut digits = Vec::with_capacity(self.digit_count() as usize);
        let mut n = self;

        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        digits
    }

    fn digit_sum(self) -> u64 {
        let mut n = self;
        let mut result: u64 = 0;

        while n > 0 {
            result += n % 10;
            n /= 10;
        }

        result
    }
}

mod digit {
    pub(super) fn two_divides(n: u64) -> bool {
        matches!(n, 0 | 2 | 4 | 6 | 8)
    }

    pub(super) fn three_divides(n: u64) -> bool {
        matches!(n, 0 | 3 | 6 | 9)
    }

    pub(super) fn four_divides(n: u64) -> bool {
        matches!(n, 0 | 4 | 8)
    }

    pub(super) fn six_divides(n: u64) -> bool {
        matches!(n, 0 | 6)
    }
}



fn main() {
    println!("{:?}", 369.divisible_by_with(strategy::By3::DigitSum));
    println!("{:?}", 369.divisible_by_with(strategy::By3::DigitClassCount));
    println!("{:?}", 369.divisible_by_with(strategy::By3::SubtractDoubleLastDigit));

    println!("{:?}", 101.divisible_by_with(strategy::By4::LastTwoDigits));
    println!("{:?}", 101.divisible_by_with(strategy::By4::TensDigitOnesDigit));
    println!("{:?}", 101.divisible_by_with(strategy::By4::TwiceTensPlusOnes));
}