use core::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
    Shl, ShlAssign, Shr, ShrAssign,
};

/// A trait for discrete, totally ordered, integer-like scalar types.
pub trait Integer:
    Sized + Copy + PartialEq + Eq + PartialOrd + Ord +
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> +
    Div<Output = Self> + Rem<Output = Self> +
    AddAssign + SubAssign + MulAssign + DivAssign + RemAssign
{
    /// The additive identity, `0`.
    const ZERO: Self;

    /// The multiplicative identity, `1`.
    const ONE: Self;

    /// The least representable value of this type.
    const MIN: Self;

    /// The greatest representable value of this type.
    const MAX: Self;

    /// The unsigned type used to represent absolute values of this type.
    type AbsoluteValueType: Unsigned;

    /// Returns `true` if `self == 0`.
    #[must_use]
    #[inline]
    fn is_zero(self) -> bool { self == Self::ZERO }

    /// Returns `true` if `self != 0`.
    #[must_use]
    #[inline]
    fn is_nonzero(self) -> bool { self != Self::ZERO }
}

/// A trait for integer types supporting bitwise operations and shifts.
pub trait BitwiseOps: Integer +
    BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> +
    Not<Output = Self> +
    Shl<u32, Output = Self> + Shr<u32, Output = Self> +
    BitAndAssign + BitOrAssign + BitXorAssign +
    ShlAssign<u32> + ShrAssign<u32>
{
    /// Returns the number of consecutive zero bits starting at the least significant bit.
    #[must_use]
    fn trailing_zeros(self) -> u32;
}

/// A marker trait for unsigned integer types.
pub trait Unsigned: Integer {}

/// A trait for signed integer types.
pub trait Signed: Integer {
    /// Returns `true` if `self > 0`.
    #[must_use]
    #[inline]
    fn is_positive(self) -> bool { self > Self::ZERO }

    /// Returns `true` if `self <= 0`.
    #[must_use]
    #[inline]
    fn is_nonpositive(self) -> bool { self <= Self::ZERO }

    /// Returns `true` if `self < 0`.
    #[must_use]
    #[inline]
    fn is_negative(self) -> bool { self < Self::ZERO }

    /// Returns `true` if `self >= 0`.
    #[must_use]
    #[inline]
    fn is_nonnegative(self) -> bool { self >= Self::ZERO }

    /// Returns the absolute value of `self`.
    ///
    /// Panics if `self == MIN`.
    #[must_use]
    #[inline]
    fn absolute(self) -> Self {
        self.strict_absolute()
    }

    /// Returns `|self|` as the corresponding unsigned type.
    ///
    /// This is defined for all values, including `MIN`.
    #[must_use]
    fn unsigned_absolute(self) -> Self::AbsoluteValueType;

    /// Returns the absolute value of `self`, or `None` if it is not representable.
    ///
    /// For two's-complement signed integers, this fails exactly when `self == MIN`.
    #[must_use]
    #[inline]
    fn checked_absolute(self) -> Option<Self> {
        if self.is_nonnegative() {
            Some(self)
        } else if self != Self::MIN {
            Some(Self::ZERO - self)
        } else {
            None
        }
    }

    /// Returns the absolute value of `self`.
    ///
    /// Panics if `self == MIN`.
    #[must_use]
    #[inline]
    fn strict_absolute(self) -> Self {
        self.checked_absolute()
            .expect("attempted to take the absolute value of the minimum signed value")
    }

    /// Returns the absolute value of `self`, wrapping on overflow.
    ///
    /// If `self == MIN`, returns `MIN`.
    #[must_use]
    #[inline]
    fn wrapping_absolute(self) -> Self {
        self.checked_absolute()
            .unwrap_or(Self::MIN)
    }

    /// Returns the absolute value of `self`, saturating on overflow.
    ///
    /// If `self == MIN`, returns `MAX`.
    #[must_use]
    #[inline]
    fn saturating_absolute(self) -> Self {
        self.checked_absolute()
            .unwrap_or(Self::MAX)
    }

    /// Returns the absolute value of `self` and a flag indicating overflow.
    ///
    /// The overflow flag is `true` exactly when `self == MIN`.
    #[must_use]
    #[inline]
    fn overflowing_absolute(self) -> (Self, bool) {
        self.checked_absolute()
            .map_or((Self::MIN, true), |x| (x, false))
    }
}

// -----------------------------------------------------------------------------
// Macro Implementations
// -----------------------------------------------------------------------------

macro_rules! impl_integer_traits {
    ($t:ty, $abs:ty) => {
        impl Integer for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const MIN: Self = <$t>::MIN;
            const MAX: Self = <$t>::MAX;

            type AbsoluteValueType = $abs;
        }

        impl BitwiseOps for $t {
            #[inline]
            fn trailing_zeros(self) -> u32 {
                self.trailing_zeros()
            }
        }
    };
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
        // For unsigned types, the absolute value type is the type itself.
        impl_integer_traits!($t, $t);
        impl Unsigned for $t {}
    };

    (@step signed $s:ty => $u:ty) => {
        // For signed types, absolute values are represented by the mapped unsigned type.
        impl_integer_traits!($s, $u);

        impl Signed for $s {
            #[inline]
            fn unsigned_absolute(self) -> Self::AbsoluteValueType {
                let bits = self as Self::AbsoluteValueType;
                if self.is_negative() {
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
