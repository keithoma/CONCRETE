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

macro_rules! impl_traits {
    ($t:ty, $mod_name:ident) => {
        impl Natural for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }

        impl Signed for $t {
            fn is_negative(self) -> bool { self < 0 as $t }
            fn abs(self) -> Self { if !self.is_negative() { self } else { Self::ZERO - self } }
        }

        impl RationalOps for $t {
            // TODO
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

impl_integer!(u8, gcd_u8);
impl_integer!(u16, gcd_u16);
impl_integer!(u32, gcd_u32);
impl_integer!(u64, gcd_u64);
impl_integer!(u128, gcd_u128);
impl_integer!(usize, gcd_usize);

impl_integer!(i8, gcd_i8);
impl_integer!(i16, gcd_i16);
impl_integer!(i32, gcd_i32);
impl_integer!(i64, gcd_i64);
impl_integer!(i128, gcd_i128);
