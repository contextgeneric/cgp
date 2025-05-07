#![no_std]

/*!
   This crate defines the core CGP traits, [`DelegateComponent`] and [`HasCgpProvider`].
*/

mod traits;
mod types;

pub use traits::{CanUseComponent, DelegateComponent, HasCgpProvider, IsProviderFor};
pub use types::{UseContext, UseDelegate, UseFields, WithContext, WithProvider};
