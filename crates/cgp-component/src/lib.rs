#![no_std]

pub mod traits;

pub use cgp_component_macro::{define_components, delegate_components, derive_component};
pub use traits::{DelegateComponent, HasComponents};
