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

pub trait Signed: Natural {
    type Unsigned: Natural;

    #[inline]
    fn is_negative(self) -> bool {
        self < Self::ZERO
    }

    #[inline] fn unsigned_abs(self) -> Self::Unsigned;

    #[inline]
    fn checked_abs(self) -> Option<Self> {
        if self >= Self::ZERO {
            Some(self)
        } else if self != Self::MIN {
            Some(Self::ZERO - self)
        } else {
            None
        }
    }

    #[inline]
    fn strict_abs(self) -> Self {
        self.checked_abs()
            .expect("attempted to take the absolute value of the minimum signed value")
    }

    #[inline] fn wrapping_abs(self) -> Self;
    #[inline] fn saturating_abs(self) -> Self;
    #[inline] fn overflowing_abs(self) -> (Self, bool);
}

macro_rules! impl_all {
    (
        $( unsigned $u:ty, )*
        $( signed $s:ty => $su:ty, )*
    ) => {
        $(
            impl_all!(@step unsigned $u);
        )*

        $(
            impl_all!(@step signed $s => $su);
        )*
    };

    (@step unsigned $t:ty) => {
        impl_unsigned_traits!($t);
    };

    (@step signed $s:ty => $u:ty) => {
        impl_unsigned_traits!($s);

        impl Signed for $s {
            type Unsigned = $u;

            fn unsigned_abs(self) -> Self::Unsigned {
                let bits = self as Self::Unsigned;
                if self < 0 {
                    (!bits) + 1
                } else {
                    bits
                }
            }
        }
    };
}

impl_all!(
    unsigned u8,
    unsigned u16,
    unsigned u32,
    unsigned u64,
    unsigned u128,
    unsigned usize,

    signed i8 => u8,
    signed i16 => u16,
    signed i32 => u32,
    signed i64 => u64,
    signed i128 => u128,
    signed isize => usize,
);
