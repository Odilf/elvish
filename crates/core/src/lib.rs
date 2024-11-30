//! Core utilities for `elvish` (used also inside macros)

#![warn(missing_docs)]

pub mod data;
pub mod solution;

// TODO: This should be an enum whenever enum const generics are possible
/// Either part 1 (0) or part 2 (1)
pub type Part = u8;
