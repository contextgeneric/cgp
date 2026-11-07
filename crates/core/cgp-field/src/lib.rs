#![no_std]
//! This crate provides CGP's field access: the field traits, the field tags, the type-level lists,
//! and the structural casts.
//!
//! See [`HasField`](https://contextgeneric.dev/docs/reference/traits/field-access/has_field), [`Index`](https://contextgeneric.dev/docs/reference/types/index_type), [`MRef`](https://contextgeneric.dev/docs/reference/types/mref), and the [casts](https://contextgeneric.dev/docs/reference/traits/casting/).

pub mod impls;
pub mod traits;
pub mod types;
