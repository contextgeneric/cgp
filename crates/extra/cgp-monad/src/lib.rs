#![no_std]
//! This crate provides CGP's monadic handlers, which compose handlers through a monad.
//!
//! See [`MonadicBind`](https://contextgeneric.dev/docs/reference/traits/monad/monadic_bind), [`ContainsValue`](https://contextgeneric.dev/docs/reference/traits/monad/contains_value), [`LiftValue`](https://contextgeneric.dev/docs/reference/traits/monad/lift_value), and [`PipeMonadic`](https://contextgeneric.dev/docs/reference/providers/monad/pipe_monadic).

pub mod monadic;
pub mod providers;
pub mod traits;
