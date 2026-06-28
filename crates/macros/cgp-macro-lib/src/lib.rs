/*!
   This is an internal crate used by the `cgp-macro` crate. We implement the
   proc macros for `cgp-component` as a library, so that it can be more easily tested.
   The constructs are then re-exported as proc macros in the `cgp-macro` crate,
   which is defined as a proc macro crate.
*/

extern crate alloc;

pub(crate) mod derive_extractor;

mod entrypoints;

pub use crate::entrypoints::*;
