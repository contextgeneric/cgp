#![no_std]

pub mod impls;
pub mod traits;
pub mod types;

pub use cgp_field_macro::{product, symbol, HasField, Product, Sum};
pub use traits::{FieldGetter, HasField, HasFieldMut, MutFieldGetter};
pub use types::{Char, Cons, Either, Field, Index, Nil, Void};
