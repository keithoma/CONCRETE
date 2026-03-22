// to-do: use turbo fish to get one unifying function
// let check = 369.is_divisible_by::<Three>();

enum DivisibleBy2Method {
    LastDigitEven,
    ModuloOperator
}

enum DivisibleBy3Method {
    DigitSum,
    DigitClassCount,
    SubtractDoubleLastDigit,
    ModuloOperator
}

enum DivisibleBy4Method {
    LastTwoDigits,
    TensDigitOnesDigit,
    TwiceTensPlusOnes,
    ModuloOperator
}

enum DivisibleBy5Method {
    LastDigit,
    ModuloOperator
}

enum DivisibleBy6Method {
    TwoAndThree,
    SumOfDigits
    ModuloOperator
}

enum DivisibleBy7Method {
    ModuloOperator
}

enum DivisibleBy8Method {
    ModuloOperator
}

enum DivisibleBy9Method {
    ModuloOperator
}

enum DivisibleBy10Method {
    ModuloOperator
}

impl Default for DivisibleBy2Method { fn default() -> Self { Self::LastDigitEven } }
impl Default for DivisibleBy3Method { fn default() -> Self { Self::DigitSum } }
impl Default for DivisibleBy4Method { fn default() -> Self { Self::LastTwoDigits } }
impl Default for DivisibleBy5Method { fn default() -> Self { Self::LastDigit } }
impl Default for DivisibleBy6Method { fn default() -> Self { Self::TwoAndThree } }

trait Divisible {
    fn divisible_by_with(self, method: impl DivisibilityByMethodTrait) -> bool;

    fn is_divisible_by<M: DivisibilityByMethodTrait + Default>(self) -> bool;
    where Self: Sized 
    {
        self.divisible_by_with(M::default())
    }
}

trait DivisibilityByMethodTrait {
    fn check(&self, x: u64) -> bool;
}

fn digit_sum(n: u64) -> u64 {
    let mut n = n;
    let mut result: u64 = 0;

    while n > 0 {
        result += n % 10;
        n /= 10;
    }

    result
}

fn get_digit(n: u64, i: u64) -> u64 { (n % (10.pow(i - 1))) / 10.pow(i - 1) }

mod digits {
    pub(super) fn divisible_by_2(n: u64) -> bool {
        matches!(n, 0 | 2 | 4 | 6 | 8)
    }

    pub(super) fn divisible_by_3(n: u64) -> bool {
        matches!(n, 0 | 3 | 6 | 9)
    }

    pub(super) fn divisible_by_4(n: u64) -> bool {
        matches!(n, 0 | 4 | 8)
    }

    pub(super) fn divisible_by_6(n: u64) -> bool {
        matches!(n, 0 | 6)
    }
}

impl DivisibilityByMethodTrait for DivisibleBy2Method {
    fn check(&self, n: u64) -> bool {
        match self {
            DivisibleBy2Method::LastDigitEven => {
                matches!(n % 10, 0 | 2 | 4 | 6 | 8)
            }

            DivisibleBy2Method::ModuloOperator => {
                n % 2 == 0
            }
        }
    }

}

impl DivisibilityByMethodTrait for DivisibleBy3Method {
    fn check(&self, x: u64) -> bool {
        match self {
            DivisibleBy3Method::DigitSum => {
                if x < 10 {
                    crate::digits::divisible_by_3(x)
                } else {
                    digit_sum(x).divisible_by_with(DivisibleBy3Method::DigitSum)
                }
            }

            DivisibleBy3Method::DigitClassCount => {
                let mut remainder = x;
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
                    crate::digits::divisible_by_3(difference)
                } else {
                    difference.divisible_by_with(DivisibleBy3Method::DigitClassCount)
                }
            }

            DivisibleBy3Method::SubtractDoubleLastDigit => {
                if x < 10 {
                    crate::digits::divisible_by_3(x)
                } else {
                    let new_x: u64 = (x / 10).abs_diff(2 * (x % 10));
                    new_x.divisible_by_with(DivisibleBy3Method::SubtractDoubleLastDigit)
                }
            }

            DivisibleBy3Method::ModuloOperator => {
                x % 3 == 0
            }

        }
    }
}

impl DivisibilityByMethodTrait for DivisibleBy4Method {
    fn check(&self, n: u64) -> bool {
        match self {
            DivisibleBy4Method::LastTwoDigits => {
                let divisible_last_two_digits: [u64; 25] = [
                    0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48,
                    52, 56, 60, 64, 68, 72, 76, 80, 84, 88, 92, 96
                ];

                let last_two_digits: u64 = n % 100;
                divisible_last_two_digits.contains(&last_two_digits)
            }

            DivisibleBy4Method::TensDigitOnesDigit => {
                let tens_digit: u64 = (n % 100) / 10;
                let ones_digit: u64 = n % 10;
                if crate::digits::divisible_by_2(tens_digit) {
                    matches!(ones_digit, 0 | 4 | 8)
                } else {
                    matches!(ones_digit, 2 | 6)
                }
            }

            DivisibleBy4Method::TwiceTensPlusOnes => {
                if n < 10 {
                    crate::digits::divisible_by_4(n)
                } else {
                    let intermediate_result: u64 = 2 * ((n % 100) / 10) + (n % 10);
                    intermediate_result.divisible_by_with(DivisibleBy4Method::TwiceTensPlusOnes)
                }
            }

            DivisibleBy4Method::ModuloOperator => {
                n % 4 == 0
            }
        }

    }

}

impl DivisibilityByMethodTrait for DivisibleBy5Method {
    fn check(&self, n: u64) -> bool {
        match self {
            DivisibleBy5Method::LastDigit => {
                matches!(n % 10, 0 | 5)
            }

            DivisibleBy5Method::ModuloOperator => {
                n % 5 == 0
            }
        }
    }
}

impl DivisibilityByMethodTrait for DivisibleBy6Method {
    fn check(&self, n: u64) -> bool {
        match self {
            DivisibleBy6Method::TwoAndThree => {
                n.is_divisible_by::<DivisibleBy2Method>() &&
                n.is_divisible_by::<DivisibleBy3Method>()
            }

            DivisibleBy6Method::SumOfDigits => {
                let intermediate: u64 = 4 * (
                    get_digit(n, 4) + get_digit(n, 3) + get_digit(n, 2)
                ) + get_digit(n, 1);

                if intermediate < 10 {
                    crate::digits::divisible_by_6(intermediate)
                } else {
                    intermediate.divisible_by_with(DivisibleBy6Method::SumOfDigits)
                }
            }

            DivisibleBy6Method::ModuloOperator => {
                n % 6 == 0
            }
        }
    }
}

impl Divisible for u64 {
    fn divisible_by_with(self, method: impl DivisibilityByMethodTrait) -> bool {
        method.check(self)
    }
}

fn main() {
    println!("{:?}", 369.divisible_by_with(DivisibleBy3Method::DigitSum));
    println!("{:?}", 369.divisible_by_with(DivisibleBy3Method::DigitClassCount));
    println!("{:?}", 369.divisible_by_with(DivisibleBy3Method::SubtractDoubleLastDigit));

    println!("{:?}", 101.divisible_by_with(DivisibleBy4Method::LastTwoDigits));
    println!("{:?}", 101.divisible_by_with(DivisibleBy4Method::TensDigitOnesDigit));
    println!("{:?}", 101.divisible_by_with(DivisibleBy4Method::TwiceTensPlusOnes));
}