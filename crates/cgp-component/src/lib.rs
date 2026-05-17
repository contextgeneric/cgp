#![no_std]

/*!
   This crate defines the core CGP types and traits used to enable the core
   CGP component implementation.
*/

pub mod macro_prelude;

mod namespaces;
mod traits;
mod types;

pub use namespaces::DefaultNamespace;
pub use traits::{CanUseComponent, DelegateComponent, IsProviderFor};
pub use types::{
    RedirectLookup, UseContext, UseDefault, UseDelegate, UseFields, WithContext, WithProvider,
};
