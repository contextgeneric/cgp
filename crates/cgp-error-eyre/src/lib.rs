#![no_std]

extern crate alloc;

mod impls;

pub use impls::{DebugEyreError, DisplayEyreError, RaiseEyreError, UseEyreError};
