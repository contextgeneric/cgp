/*!
   This is an internal crate used by the `cgp-macro` crate. We implement the
   proc macros for `cgp-component` as a library, so that it can be more easily tested.
   The constructs are then re-exported as proc macros in the `cgp-macro` crate,
   which is defined as a proc macro crate.
*/

extern crate alloc;

pub(crate) mod derive_builder;
pub(crate) mod derive_extractor;
pub(crate) mod derive_has_fields;
pub(crate) mod field;
pub(crate) mod type_component;

mod entrypoints;

pub use field::derive_has_field;

pub use crate::entrypoints::*;
