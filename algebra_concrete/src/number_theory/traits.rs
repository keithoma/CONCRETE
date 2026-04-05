use crate::structures::integer::{Integer, Signed, Unsigned};
use crate::number_theory::gcd::{GcdAlgorithm, gcd, lcm};

pub trait RationalOps: Integer {
    type UnsignedType: Unsigned;

    fn gcd(self, other: Self) -> Self::UnsignedType;

    fn gcd_with<Algo>(self, other: Self) -> Self::UnsignedType
    where
        Algo: GcdAlgorithm<Self::UnsignedType>;


    fn lcm(self, other: Self) -> Self::UnsignedType;
}

macro_rules! impl_unsigned_math {
    ($($t:ty => $default_algo:ty),*) => {
        $(
            impl RationalOps for $t {
                type UnsignedType = $t;

                #[inline]
                fn gcd(self, other: Self) -> Self::UnsignedType {
                    self.gcd_with::<$default_algo>(other)
                }

                #[inline]
                fn gcd_with<Algo>(self, other: Self) -> Self::UnsignedType
                where
                    Algo: GcdAlgorithm<Self::UnsignedType>
                {
                    Algo::compute(self, other)
                }
            }
        )*
    };
}

impl_unsigned_math!(
    u8 => SteinIterative,
    u16 => SteinIterative,
    u32 => SteinIterative,
    u64 => SteinIterative,
    u128 => SteinIterative,
    usize => SteinIterative
);

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

impl_signed_math!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, i128 => u128, isize => usize);