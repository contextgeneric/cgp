#![no_std]
#![allow(clippy::needless_lifetimes)]
//! This crate provides CGP's dispatch providers, which match, extract, and build a value on the way
//! to a handler.
//!
//! See [`MatchWithHandlers`](https://contextgeneric.dev/docs/reference/providers/dispatch/match_with_handlers), [`MatchWithValueHandlers`](https://contextgeneric.dev/docs/reference/providers/dispatch/match_with_value_handlers), and [`ExtractFieldAndHandle`](https://contextgeneric.dev/docs/reference/providers/dispatch/extract_field_and_handle).
#![warn(missing_docs)]

mod providers;

pub use providers::*;
