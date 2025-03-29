// #![no_std]

/*!
   This is an internal crate used by the `cgp-macro` crate. We implement the
   proc macros for `cgp-component` as a library, so that it can be more easily tested.
   The constructs are then re-exported as proc macros in the `cgp-macro` crate,
   which is defined as a proc macro crate.
*/

extern crate alloc;

pub(crate) mod blanket_trait;
pub(crate) mod check_components;
pub(crate) mod delegate_components;
pub(crate) mod derive_component;
pub(crate) mod derive_context;
pub(crate) mod derive_getter;
pub(crate) mod derive_has_fields;
pub(crate) mod derive_provider;
mod entrypoints;
pub(crate) mod field;
pub(crate) mod for_each_replace;
pub(crate) mod parse;
pub(crate) mod preset;
pub(crate) mod product;
pub(crate) mod symbol;
pub(crate) mod type_component;

pub use field::derive_fields;
pub use product::{make_product_expr, make_product_type, make_sum_type};
pub use symbol::make_symbol;

#[cfg(test)]
mod tests;

pub use crate::entrypoints::*;
