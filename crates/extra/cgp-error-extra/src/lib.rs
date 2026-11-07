#![no_std]
//! This crate provides CGP's generic error providers, which raise, return, discard, and format an
//! error. The `DebugError` and `DisplayError` providers need an allocator, so they are behind the
//! `alloc` feature.
//!
//! See [`RaiseFrom`](https://contextgeneric.dev/docs/reference/providers/error/raise_from), [`ReturnError`](https://contextgeneric.dev/docs/reference/providers/error/return_error), [`RaiseInfallible`](https://contextgeneric.dev/docs/reference/providers/error/raise_infallible), [`PanicOnError`](https://contextgeneric.dev/docs/reference/providers/error/panic_on_error), [`DiscardDetail`](https://contextgeneric.dev/docs/reference/providers/error/discard_detail), [`DebugError`](https://contextgeneric.dev/docs/reference/providers/error/debug_error), and [`DisplayError`](https://contextgeneric.dev/docs/reference/providers/error/display_error).

#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;

#[cfg(feature = "alloc")]
pub use impls::{DebugError, DisplayError};
pub use impls::{DiscardDetail, PanicOnError, RaiseFrom, RaiseInfallible, ReturnError};
