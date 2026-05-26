#![no_std]

/*!
   This crate defines the core CGP types and traits used to enable the core
   CGP component implementation.
*/

pub mod macro_prelude;

mod namespaces;
mod providers;
mod traits;

pub use namespaces::{DefaultImpls1, DefaultImpls2, DefaultNamespace};
pub use providers::{
    RedirectLookup, UseContext, UseDefault, UseDelegate, UseFields, WithContext, WithProvider,
};
pub use traits::{CanUseComponent, DelegateComponent, IsProviderFor};
