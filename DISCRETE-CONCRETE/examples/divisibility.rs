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


trait Divisible {
    fn divisible_by_with(self, n: u64, method: impl DivMethodTrait) -> bool;
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

mod digits {
    pub(super) fn is_divisible_by_3_digit(n: u64) -> bool {
        matches!(n, 0 | 3 | 6 | 9)
    }
}

impl DivisibilityByMethodTrait for DivisibleBy2Method {
    fn check(&self, x: u64) -> bool {
        match self {
            DivisibleBy2Method::LastDigitEven => {
                matches!(x % 10, 0 | 2 | 4 | 6 | 8)
            }

            DivisibleBy2Method::ModuloOperator => {
                x % 2 == 0
            }
        }
    }

}

impl DivisibilityByMethodTrait for DivisibleBy3Method {
    fn check(&self, x: u64) -> bool {
        match self {
            DivisibleBy3Method::DigitSum => {
                if x < 10 {
                    matches!(x, 0 | 3 | 6 | 9) // since I have the same code below, this should become helpers
                } else {
                    digit_sum(x).divisible_by_3_with(DivisibleBy3Method::DigitSum)
                }
            }

            DivisibleBy3Method::DigitClassCount => {
                let mut y = x;
                let mut quantity_1_4_7: u64 = 0;
                let mut quantity_2_5_8: u64 = 0;

                while y > 0 {
                    let digit: u64 = y % 10;
                    if matches!(digit, 1 | 4 | 7) {
                        quantity_1_4_7 += 1;
                    } else if matches!(digit, 2 | 5 | 8) {
                        quantity_2_5_8 += 1;
                    }
                    y /= 10;
                }

                let difference: u64 = if quantity_1_4_7 >= quantity_2_5_8 {
                    quantity_1_4_7 - quantity_2_5_8
                } else {
                    quantity_2_5_8 - quantity_1_4_7
                }

                if difference < 10 {
                    is_divisible_by_3_digit(difference)
                } else {
                    difference.divisible_by_with(DivisibleBy3Method::DigitClassCount)
                }
            }

            DivisibleBy3Method::SubtractDoubleLastDigit => {
                if x < 10 {
                    is_divisible_by_3_digit(x)
                } else {
                    let new_x: u64 = x - 2 * (x % 10);
                    divisible_by_with(3, DivisibleBy3Method::SubtractDoubleLastDigit)
                }
            }

            DivisibleBy3Method::ModuloOperator => {
                x % 3 == 0
            }

        }
    }
}


// should be
// trait Divisible {
//     fn divisible_by(self, n: u64) -> bool;
// }

trait Divisible {
    fn divisible_by_2(self) -> bool;
    fn divisible_by_2_with(self, method: DivisibleBy2Method) -> bool;

    fn divisible_by_3(self) -> bool;
    fn divisible_by_3_with(self, method: DivisibleBy3Method) -> bool;
    
    fn divisible_by_4(self) -> bool;
    fn divisible_by_5(self) -> bool;
    fn divisible_by_6(self) -> bool;
    // fn divisible_by_7(self) -> bool;
    // fn divisible_by_8(self) -> bool;
    fn divisible_by_9(self) -> bool;
    fn divisible_by_10(self) -> bool;
}

mod digits {
    pub(super) fn is_divisible_by_3_digit(n: u64) -> bool {
        matches!(n, 0 | 3 | 6 | 9)
    }
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

impl Divisible for u64 {
    fn divisible_by_2(self) -> bool {
        self.divisible_by_2_with(DivisibleBy2Method::LastDigitEven)
    }

    fn divisible_by_2_with(self, method: DivisibleBy2Method) -> bool {
        match method {
            DivisibleBy2Method::LastDigitEven => {
                matches!(self % 10, 0 | 2 | 4 | 6 | 8)
            }

            DivisibleBy2Method::ModuloOperator => {
                self % 2 == 0
            }
        }
    }

    fn divisible_by_3(self) -> bool {
        self.divisible_by_3_with(DivisibleBy3Method::DigitSum)
    }

    fn divisible_by_3_with(self, method: DivisibleBy3Method) -> bool {
        match method {
            DivisibleBy3Method::DigitSum => {
                if self < 10 {
                    matches!(self, 0 | 3 | 6 | 9) // since I have the same code below, this should become helpers
                } else {
                    digit_sum(self).divisible_by_3_with(DivisibleBy3Method::DigitSum)
                }
            }

            DivisibleBy3Method::DigitClassCount => {
                let mut n = self;
                let mut quantity_2_5_8: u64 = 0;
                let mut quantity_1_4_7: u64 = 0;

                while n > 0 {
                    let digit: u64 = n % 10;
                    if matches!(digit, 2 | 5 | 8) {
                        quantity_2_5_8 += 1;
                    } else if matches!(digit, 1 | 4 | 7) {
                        quantity_1_4_7 += 1;
                    }
                    n /= 10;
                }
                let difference: u64 = (quantity_2_5_8 - quantity_1_4_7).abs();

                if differerence < 10 {
                    matches!(difference, 0 | 3 | 6 | 9)
                } else {
                    difference.divisible_by_3_with(DigitClassCount)
                }
            }

            DivisibleBy3Method::SubtractDoubleLastDigit => {
                // implement this
                false
            }

            DivisibleBy3Method::ModuloOperator => {
                self % 3 == 0
            }
        }
    }

    fn divisible_by_4(self) -> bool {
        matches!(self % 100, 0 | 4 | 8 | 12 | 16 | 20 | 24 | 28 | 32 | 36 | 40 | 44 | 48 | 52 | 56 | 60 | 64 | 68 | 72 | 76 | 80 | 84 | 88 | 92 | 96 )
    }

    fn divisible_by_5(self) -> bool {
        matches!(self % 10, 0 | 5)
    }

    fn divisible_by_6(self) -> bool {
        self.divisible_by_2() && self.divisible_by_3()
    }

    fn divisible_by_9(self) -> bool {
        if self < 10 {
            self == 9
        } else {
            digit_sum(self).divisible_by_9()
        }
    }

    fn divisible_by_10(self) -> bool {
        self % 10 == 0
    }
}

fn main() {
    println!("{:?}", 369.divisible_by_3())
}