#![doc = include_str!("../README.md")]
#![no_std]
#![warn(missing_docs)]

/// The field traits' implementations and the built-in providers.
pub mod impls;
/// The field traits: access, building, casting, and the type-level lists.
pub mod traits;
/// The field types: tags, lists, sums, and the owned-or-borrowed value.
pub mod types;
