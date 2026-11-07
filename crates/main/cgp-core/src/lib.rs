#![no_std]
#![allow(mixed_script_confusables)]
//! This crate re-exports CGP's core layer through `cgp::core`: the base layer, the macros, the
//! async trait macro, and the error, field, and type components.
//!
//! See [`#[cgp_component]`](https://contextgeneric.dev/docs/reference/macros/cgp_component), [`HasField`](https://contextgeneric.dev/docs/reference/traits/field-access/has_field), and [`HasErrorType`](https://contextgeneric.dev/docs/reference/components/has_error_type).
#![warn(missing_docs)]

/// The prelude, re-exported as `cgp_core::prelude` and `cgp_core::macro_prelude`.
pub mod prelude;

pub use prelude as macro_prelude;
#[doc(inline)]
pub use {
    cgp_async_macro::async_trait, cgp_base as base, cgp_component as component, cgp_error as error,
    cgp_field as field, cgp_macro as macros, cgp_type as types,
};
