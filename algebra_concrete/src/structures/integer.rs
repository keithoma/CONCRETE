use core::ops::Add;
use core::ops::Sub;

/// missing
pub trait Integer:
    Sized + Copy +
    Add<Output = Self> +
    Sub<Output = Self> +
{
    /// missing
    const ZERO: Self;

    /// missing
    const ONE: Self;
}

impl Integer for i32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Integer for u64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}