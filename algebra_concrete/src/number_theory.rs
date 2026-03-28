mod gcd;

pub use gcd::GcdStrategy;

// Unsigned Integers
pub use gcd::gcd_u8::{gcd as gcd_u8, gcd_with_strategy as gcd_with_strategy_u8};
pub use gcd::gcd_u16::{gcd as gcd_u16, gcd_with_strategy as gcd_with_strategy_u16};
pub use gcd::gcd_u32::{gcd as gcd_u32, gcd_with_strategy as gcd_with_strategy_u32};
pub use gcd::gcd_u64::{gcd as gcd_u64, gcd_with_strategy as gcd_with_strategy_u64};
pub use gcd::gcd_u128::{gcd as gcd_u128, gcd_with_strategy as gcd_with_strategy_u128};
pub use gcd::gcd_usize::{gcd as gcd_usize, gcd_with_strategy as gcd_with_strategy_usize};

// Signed Integers
pub use gcd::gcd_i8::{gcd as gcd_i8, gcd_with_strategy as gcd_with_strategy_i8};
pub use gcd::gcd_i16::{gcd as gcd_i16, gcd_with_strategy as gcd_with_strategy_i16};
pub use gcd::gcd_i32::{gcd as gcd_i32, gcd_with_strategy as gcd_with_strategy_i32};
pub use gcd::gcd_i64::{gcd as gcd_i64, gcd_with_strategy as gcd_with_strategy_i64}; 
pub use gcd::gcd_i128::{gcd as gcd_i128, gcd_with_strategy as gcd_with_strategy_i128};