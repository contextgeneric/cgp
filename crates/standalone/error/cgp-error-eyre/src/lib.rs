#![doc = include_str!("../README.md")]
// eyre needs `std`, so unlike the other backends this crate is not `no_std`.
extern crate alloc;

mod impls;

pub use eyre::Error;
pub use impls::{DebugEyreError, DisplayEyreError, RaiseEyreError, UseEyreError};
