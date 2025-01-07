#![no_std]

/*!
   This crate defines the core CGP traits, [`DelegateComponent`] and [`HasComponents`].
*/

mod traits;
mod types;

pub use traits::{DelegateComponent, HasComponents};
pub use types::{UseContext, UseDelegate, WithContext, WithProvider};
