trait Divisible {
    fn divisible_by_2(self) -> bool;
    fn divisible_by_3(self) -> bool;
    fn divisible_by_4(self) -> bool;
    fn divisible_by_5(self) -> bool;
    fn divisible_by_6(self) -> bool;
    fn divisible_by_7(self) -> bool;
    fn divisible_by_8(self) -> bool;
    fn divisible_by_9(self) -> bool;
    fn divisible_by_10(self) -> bool;
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
        matches!(self % 10, 0 | 2 | 4 | 6 | 8)
    }

    fn divisible_by_3(self) -> bool {
        if self < 10 {
            return matches!(self, 0 | 3 | 6 | 9);
        } else {
            digit_sum(self).divisible_by_3()
        }
    }

    fn by_4(self) -> bool {
        matches!(self % 100, 0 | 4 | 8 | 12 | 16 | 20 | 24 | 28 | 32 | 36 | 40 | 44 | 48 | 52 | 56 | 60 | 64 | 68 | 72 | 76 | 80 | 84 | 88 | 92 | 96 )
    }

    fn by_5(self) -> bool {
        matches!(self % 10, 0 | 5)
    }

    fn by_6(self) -> bool {
        self.divisible_by_2 && self.divisible_by_3
    }

    fn by_9(self) -> bool {
        if self < 10 {
            return self == 9;
        } else {
            digit_sum(self).divisible_by_9()
        }
    }

    fn by_10(self) -> bool {
        self % 10 == 0
    }
}

fn main() {
    println!("{:?}", 369.divisible_by_3())
}