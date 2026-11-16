#![doc = include_str!("../README.md")]
#![no_std]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;

#[cfg(feature = "alloc")]
pub use impls::{DebugError, DisplayError};
pub use impls::{DiscardDetail, PanicOnError, RaiseFrom, RaiseInfallible, ReturnError};
