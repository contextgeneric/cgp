#![no_std]

/*!
   This crate defines the core CGP traits, [`DelegateComponent`] and [`HasComponents`].
   It also re-export component macros provided by [`cgp_component_macro`].
*/

pub mod traits;

pub use cgp_component_macro::{define_components, delegate_components, derive_component};
pub use traits::{DelegateComponent, HasComponents};
