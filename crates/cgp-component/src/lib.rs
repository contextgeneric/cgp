#![no_std]

/*!
   This crate defines the core CGP traits, [`DelegateComponent`] and [`HasComponents`].
*/

mod traits;
mod types;

pub use traits::{CanUseComponent, DelegateComponent, HasComponents, IsProviderFor};
pub use types::{UseContext, UseDelegate, UseFields, WithContext, WithProvider};
