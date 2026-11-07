#![no_std]
//! This crate provides CGP's abstract types: a type the context chooses behind a trait.
//!
//! See [`HasType`](https://contextgeneric.dev/docs/reference/components/has_type), [`UseType`](https://contextgeneric.dev/docs/reference/providers/use_type), and [`UseDelegatedType`](https://contextgeneric.dev/docs/reference/providers/use_delegated_type).
#![warn(missing_docs)]

mod impls;
mod traits;

pub use impls::{UseDelegatedType, UseType, WithDelegatedType, WithType};
pub use traits::{HasType, TypeOf, TypeProvider, TypeProviderComponent};
