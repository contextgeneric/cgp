#![no_std]

pub mod traits;
pub mod types;

pub use cgp_field_macro::derive_fields;
pub use cgp_field_macro::symbol;
pub use traits::HasField;
pub use types::Char;
