#![doc = include_str!("../README.md")]
#![no_std]
#![warn(missing_docs)]

/// Re-exports the type-level primitives that the macros expand to.
/// Only the macros use this module; import from `cgp::prelude` or the defining module instead.
pub mod macro_prelude;
/// The type-level traits: [`traits::StaticFormat`] and [`traits::ConcatPath`].
pub mod traits;
/// The type-level types: [`types::Symbol`], [`types::Chars`], [`types::Cons`], [`types::Nil`], and
/// [`types::PathCons`].
pub mod types;
