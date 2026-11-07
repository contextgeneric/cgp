#![doc = include_str!("../README.md")]
#![no_std]
#![warn(missing_docs)]

extern crate alloc;

mod impls;

pub use anyhow::Error;
pub use impls::{DebugAnyhowError, DisplayAnyhowError, RaiseAnyhowError, UseAnyhowError};
