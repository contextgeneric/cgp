// #![no_std]

/*!
   This is an internal crate used by the `cgp-component-macro` crate. We implement the
   proc macros for `cgp-component` as a library, so that it can be more easily tested.
   The constructs are then re-exported as proc macros in the `cgp-component-macro` crate,
   which is defined as a proc macro crate.
*/

extern crate alloc;

pub(crate) mod check_components;
pub(crate) mod delegate_components;
pub(crate) mod derive_component;
pub(crate) mod derive_context;
pub(crate) mod derive_provider;
mod entrypoints;
pub(crate) mod for_each_replace;
pub(crate) mod getter_component;
pub(crate) mod parse;
pub(crate) mod preset;
pub(crate) mod type_component;

#[cfg(test)]
mod tests;

pub use crate::entrypoints::*;
