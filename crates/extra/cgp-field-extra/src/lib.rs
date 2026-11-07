#![no_std]
//! This crate provides CGP's optional-field layer, for a builder whose fields are set in any order.
//!
//! See [`HasOptionalBuilder`](https://contextgeneric.dev/docs/reference/traits/optional/has_optional_builder), [`ToOptional`](https://contextgeneric.dev/docs/reference/traits/optional/to_optional), [`SetOptional`](https://contextgeneric.dev/docs/reference/traits/optional/set_optional), and [`FinalizeOptional`](https://contextgeneric.dev/docs/reference/traits/optional/finalize_optional).

pub mod impls;
