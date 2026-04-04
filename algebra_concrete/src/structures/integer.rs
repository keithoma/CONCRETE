use core::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
    Shl, ShlAssign, Shr, ShrAssign,
};

pub trait Natural: 
    Sized + Copy + PartialEq + Eq + PartialOrd + Ord +
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + 
    Div<Output = Self> + Rem<Output = Self> +
    AddAssign + SubAssign + MulAssign + DivAssign + RemAssign 
{
    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
}

pub trait Signed: Natural {
    #[inline]
    fn is_negative(self) -> bool {
        self < Self::ZERO
    }

    #[inline]
    fn abs(self) -> Self {
        if self.is_negative() {
            Self::ZERO - self
        } else {
            self
        }
    }
}

pub trait BitwiseOps: Natural +
    BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> +
    Not<Output = Self> + 
    Shl<u32, Output = Self> + Shr<u32, Output = Self> +
    BitAndAssign + BitOrAssign + BitXorAssign + 
    ShlAssign<u32> + ShrAssign<u32>
{
    fn trailing_zeros(self) -> u32;
}

pub trait RationalOps: Natural {
    fn gcd(self, other: Self) -> Self;
    fn lcm(self, other: Self) -> Self;
}

macro_rules! impl_unsigned_traits {
    ($t:ty) => {
        impl Natural for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const MIN: Self = <$t>::MIN;
            const MAX: Self = <$t>::MAX;
        }

        impl BitwiseOps for $t {
            #[inline] fn trailing_zeros(self) -> u32 { self.trailing_zeros() }
        }

        impl RationalOps for $t {
            #[inline] fn gcd(self, other: Self) -> Self { crate::number_theory::gcd(self, other) }
            #[inline] fn lcm(self, other: Self) -> Self { crate::number_theory::lcm(self, other) }
        }
    };
}

macro_rules! impl_all {
    ($( $sign:ident $t:ty ),* $(,)?) => {
        $(
            impl_all!(@step $sign $t);
        )*
    };
    
    (@step unsigned $t:ty) => {
        impl_unsigned_traits!($t);
    };
    
    (@step signed $t:ty) => {
        impl_unsigned_traits!($t);
        impl Signed for $t {}
    };
}

impl_all!(
    unsigned u8, unsigned u16, unsigned u32, unsigned u64, unsigned u128, unsigned usize,
    signed i8,   signed i16,   signed i32,   signed i64,   signed i128,   signed isize,
);
