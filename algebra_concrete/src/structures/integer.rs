use core::cmp;
use core::ops;

use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Div;
use core::ops::Rem;

/// Integer
pub trait Integer: Sized + Copy + PartialEq + Eq + PartialOrd + Ord 
where 
    Self: 
        Add<Output = Self> + 
        Sub<Output = Self> + 
        Mul<Output = Self> + 
        Div<Output = Self> + 
        Rem<Output = Self>
{
    /// The additive identity (0)
    const ZERO: Self;

    /// The multiplicative identity (1)
    const ONE: Self;

    /// Returns the greatest common divisor of self and other.
    #[must_use]
    fn gcd(self, other: Self) -> Self;
}

macro_rules! impl_integer {
    ($t:ty, $mod_name:ident) => {
        impl Integer for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;

            fn gcd(self, other: Self) -> Self {
                let mut a = self as $t;
                let mut b = other as $t;

                if a == 0 {
                    return b;
                }
                if b == 0 {
                    return a;
                }

                let a_zeros = a.trailing_zeros();
                a >>= a_zeros;

                let b_zeros = b.trailing_zeros();
                b >>= b_zeros;

                let common_zeros = if a_zeros < b_zeros { a_zeros } else { b_zeros };

                loop {
                    if a > b {
                        (a, b) = (b, a);
                    }
                    if let Some(diff) = b.checked_sub(a) {
                        b = diff;
                    }
                    if b == 0 {
                        break;
                    }
                    b >>= b.trailing_zeros();
                }

                a << common_zeros
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
