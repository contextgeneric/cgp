#![no_std]

mod impls;
mod traits;

pub use impls::{UseDelegatedType, UseType, WithDelegatedType, WithType};
pub use traits::{HasType, ProvideType, TypeComponent, TypeOf};
