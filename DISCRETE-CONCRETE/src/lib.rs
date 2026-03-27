#![no_std]
//! # Discrete Concrete
//!
//! A library for discrete mathematical operations, focusing on integer 
//! properties, digit manipulation, and number theory.

pub mod digits;

// Re-export for convenience: 
// Allows users to do `use discrete_concrete::Digits;` 
// instead of `use discrete_concrete::digits::Digits;`
pub use digits::{DigitIter, Digits};