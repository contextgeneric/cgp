#![no_std]

pub mod traits;

pub use cgp_component_macro::{derive_component, delegate_components, define_components};
pub use traits::{DelegateComponent, HasComponents};
