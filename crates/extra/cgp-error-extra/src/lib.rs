#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;

#[cfg(feature = "alloc")]
pub use impls::{DebugError, DisplayError};
pub use impls::{DiscardDetail, PanicOnError, RaiseFrom, RaiseInfallible, ReturnError};
