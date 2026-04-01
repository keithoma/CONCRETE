//! # ALGEBRA-CONCRETE
//!
//! Algebra is the study of abstract mathematical structures—such as groups,
//! rings, fields, and lattices—and the formal rules governing the operations
//! and relations between their elements, independent of the specific objects
//! they represent.
#![no_std]
#![warn(missing_docs)]

/// A module concerned with number theory.
///
/// Number theory is a branch of pure mathematics devoted to the study of
/// integers and their intricate properties, focusing heavily on concepts like
/// prime numbers, divisibility, and solving equations using whole numbers.
pub mod number_theory;

/// A module concerned with
///
/// Algebraic structures are mathematical sets equipped with one or more
/// operations—such as addition or multiplication—that follow specific logical
/// rules known as axioms.
pub mod structures;

pub use structures::integer::{Natural, Signed, BitwiseOps, RationalOps};