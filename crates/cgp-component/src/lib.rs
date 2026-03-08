#![no_std]

/*!
   This crate defines the core CGP types and traits used to enable the core
   CGP component implementation.
*/

mod namespaces;
mod traits;
mod types;

pub use namespaces::{CoreComponents, DefaultNamespace};
pub use traits::{CanUseComponent, DelegateComponent, IsProviderFor};
pub use types::{RedirectLookup, UseContext, UseDelegate, UseFields, WithContext, WithProvider};
