#![no_std]

pub mod impls;
pub mod traits;
pub mod types;

pub use cgp_field_macro::{symbol, HasField};
pub use traits::{HasField, HasFieldMut};
pub use types::Char;
