#![no_std]

/*!
   This crate defines the core CGP types and traits used to enable the core
   CGP component implementation.
*/

mod traits;
mod types;

pub use traits::{CanUseComponent, DelegateComponent, HasCgpProvider, IsProviderFor};
pub use types::{UseContext, UseDelegate, UseFields, WithContext, WithProvider};
