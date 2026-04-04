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
}

pub trait Signed: Natural {
    fn is_negative(self) -> bool;
    fn abs(self) -> Self; 
}

pub trait RationalOps: Natural {
    fn gcd(self, other: Self) -> Self;
    fn lcm(self, other: Self) -> Self;
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

macro_rules! impl_unsigned_traits {
    ($t:ty) => {
        impl Natural for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }

        impl RationalOps for $t {
            fn gcd(self, other: Self) -> Self { self }
            fn lcm(self, other: Self) -> Self { self }
        }

        impl BitwiseOps for $t {
            #[inline]
            fn trailing_zeros(mut self) -> u32 {
                self.trailing_zeros()
            }
        }
    }
}

macro_rules! impl_signed_traits {
    ($t:ty) => {
        impl Signed for $t {
            #[inline]
            fn is_negative(self) -> bool { self < 0 as $t }

            #[inline]
            fn abs(self) -> Self { if !self.is_negative() { self } else { Self::ZERO - self } }
        }        
    };
}

impl_unsigned_traits!(u8);
impl_unsigned_traits!(u16);
impl_unsigned_traits!(u32);
impl_unsigned_traits!(u64);
impl_unsigned_traits!(u128);
impl_unsigned_traits!(usize);

impl_unsigned_traits!(i8);
impl_unsigned_traits!(i16);
impl_unsigned_traits!(i32);
impl_unsigned_traits!(i64);
impl_unsigned_traits!(i128);

impl_signed_traits!(i8);
impl_signed_traits!(i16);
impl_signed_traits!(i32);
impl_signed_traits!(i64);
impl_signed_traits!(i128);
