/*!
   This is an internal crate used by the `cgp-component-macro` crate. We implement the
   proc macros for `cgp-component` as a library, so that it can be more easily tested.
   The constructs are then re-exported as proc macros in the `cgp-component-macro` crate,
   which is defined as a proc macro crate.
*/

pub mod delegate_components;
pub mod derive_component;

#[cfg(test)]
mod tests;

pub use crate::delegate_components::{define_components, delegate_components};
pub use crate::derive_component::derive_component;
