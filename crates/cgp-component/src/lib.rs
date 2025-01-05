#![no_std]

/*!
   This crate defines the core CGP traits, [`DelegateComponent`] and [`HasComponents`].
*/

pub mod traits;
pub mod types;

pub use traits::{DelegateComponent, HasComponents};
pub use types::{UseContext, UseDelegate, WithContext, WithProvider};
