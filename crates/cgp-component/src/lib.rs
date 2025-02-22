#![no_std]

/*!
   This crate defines the core CGP traits, [`DelegateComponent`] and [`HasProvider`].
*/

mod traits;
mod types;

pub use traits::{CanUseComponent, DelegateComponent, HasProvider, IsProviderFor};
pub use types::{UseContext, UseDelegate, UseFields, WithContext, WithProvider};
