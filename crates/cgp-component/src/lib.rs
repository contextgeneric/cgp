#![no_std]

/*!
   This crate defines the core CGP types and traits used to enable the core
   CGP component implementation.
*/

mod namespaces;
mod traits;
mod types;

pub use namespaces::*;
pub use traits::*;
pub use types::*;
