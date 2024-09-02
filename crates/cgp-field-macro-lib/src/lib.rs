/*!
   This is an internal crate used by the `cgp-field-macro` crate. We implement the
   proc macros for `cgp-field` as a library, so that it can be more easily tested.
   The constructs are then re-exported as proc macros in the `cgp-field-macro` crate,
   which is defined as a proc macro crate.
*/

pub mod field;
pub mod symbol;

#[cfg(test)]
mod tests;

pub use field::derive_fields;
pub use symbol::make_symbol;
