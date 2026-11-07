#![no_std]
//! This crate provides CGP's error components: the abstract error type, raising, and wrapping.
//!
//! See [`HasErrorType`](https://contextgeneric.dev/docs/reference/components/has_error_type), [`CanRaiseError`](https://contextgeneric.dev/docs/reference/components/can_raise_error), and [`CanWrapError`](https://contextgeneric.dev/docs/reference/components/can_wrap_error).
#![warn(missing_docs)]

mod contexts;
mod traits;

pub use contexts::*;
pub use traits::*;
