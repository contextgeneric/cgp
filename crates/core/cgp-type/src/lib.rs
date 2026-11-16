#![doc = include_str!("../README.md")]
#![no_std]
#![warn(missing_docs)]

mod impls;
mod traits;

pub use impls::{UseDelegatedType, UseType, WithDelegatedType, WithType};
pub use traits::{HasType, TypeOf, TypeProvider, TypeProviderComponent};
