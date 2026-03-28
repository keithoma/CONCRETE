use core::ops::{Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};

pub trait Integer: 
    Sized + Copy + PartialEq + Eq + PartialOrd + Ord +
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + 
    Div<Output = Self> + Rem<Output = Self> +
    AddAssign + SubAssign + MulAssign + DivAssign + RemAssign
{
    const ZERO: Self;
    const ONE: Self;

    fn is_negative(self) -> bool;
    fn abs(self) -> Self;
    
    // Placeholder
    fn gcd(self, other: Self) -> Self;
    fn lcm(self, other: Self) -> Self;
}

macro_rules! impl_integer {
    ($t:ty, $mod_name:ident) => {
        impl Integer for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;

            fn is_negative(self) -> bool { self < 0 as $t }
            fn abs(self) -> Self { if !self.is_negative() { self } else { Self::ZERO - self } }

            // TODO
            fn gcd(self, other: Self) -> Self { self }
            fn lcm(self, other: Self) -> Self { self }
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
