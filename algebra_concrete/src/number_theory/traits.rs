use crate::structures::integer::{Integer, Signed, Unsigned};
use crate::number_theory::gcd::{gcd, lcm};

pub trait RationalOps: Integer {
    type UnsignedType: Unsigned;

    fn gcd(self, other: Self) -> Self::AbsoluteValueType;
    fn lcm(self, other: Self) -> Self::AbsoluteValueType;
}

macro_rules! impl_unsigned_math {
    ($($t:ty),*) => {
        $(
            impl IntegerMath for $t {
                type UnsignedType = $t;
                
                #[inline]
                fn gcd(self, other: Self) -> Self::UnsignedType {
                    stein_iterative(self, other) // Direct call to your best algorithm
                }
                // ... lcm implementation ...
            }
        )*
    };
}

macro_rules! impl_signed_math {
    ($($s:ty => $u:ty),*) => {
        $(
            impl IntegerMath for $s {
                type UnsignedType = $u;
                
                #[inline]
                fn gcd(self, other: Self) -> Self::UnsignedType {
                    // Mathematically: the GCD of a and b is the positive generator of the ideal (a, b)
                    let a = self.unsigned_absolute();
                    let b = other.unsigned_absolute();
                    a.gcd(b) // delegates to the unsigned implementation
                }
            }
        )*
    };
}

impl_unsigned_math!(u8, u16, u32, u64, u128, usize);
impl_signed_math!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, i128 => u128, isize => usize);