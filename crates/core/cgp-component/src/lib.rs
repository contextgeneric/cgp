#![no_std]

/*!
   This crate defines the core CGP types and traits used to enable the core
   CGP component implementation.
*/

pub mod macro_prelude;

mod namespaces;
mod providers;
mod traits;

pub use namespaces::*;
pub use providers::*;
pub use traits::*;
