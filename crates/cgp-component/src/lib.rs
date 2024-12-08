#![no_std]

/*!
   This crate defines the core CGP traits, [`DelegateComponent`] and [`HasComponents`].
   It also re-export component macros provided by [`cgp_component_macro`].
*/

pub mod traits;
pub mod types;

pub use cgp_component_macro::{
    cgp_component, cgp_preset, delegate_components, for_each_replace, replace_with,
};
pub use traits::{DelegateComponent, HasComponents};
pub use types::{UseContext, UseDelegate, WithContext, WithProvider};
