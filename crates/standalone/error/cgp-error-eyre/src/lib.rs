#![no_std]

extern crate alloc;

mod impls;

pub use eyre::Error;
pub use impls::{DebugEyreError, DisplayEyreError, RaiseEyreError, UseEyreError};
