#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;

pub use impls::{DiscardDetail, PanicOnError, RaiseFrom, RaiseInfallible, ReturnError};

#[cfg(feature = "alloc")]
pub use impls::{DebugError, DisplayError};
