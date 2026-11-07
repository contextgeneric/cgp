#![no_std]
//! This crate provides CGP's monadic handlers, which compose handlers through a monad.
//!
//! See [`MonadicBind`](https://contextgeneric.dev/docs/reference/traits/monad/monadic_bind), [`ContainsValue`](https://contextgeneric.dev/docs/reference/traits/monad/contains_value), [`LiftValue`](https://contextgeneric.dev/docs/reference/traits/monad/lift_value), and [`PipeMonadic`](https://contextgeneric.dev/docs/reference/providers/monad/pipe_monadic).
#![warn(missing_docs)]

/// The monads: identity, `Ok`, and `Err`.
pub mod monadic;
/// The monadic providers, such as [`providers::PipeMonadic`].
pub mod providers;
/// The monad traits: bind, lift, transform, and value containment.
pub mod traits;
