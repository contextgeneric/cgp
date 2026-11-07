#![doc = include_str!("../README.md")]
#![no_std]
#![warn(missing_docs)]

extern crate alloc;

mod impls;
mod types;

pub use impls::{DebugBoxedStdError, DisplayBoxedStdError, RaiseBoxedStdError, UseBoxedStdError};
pub use types::{Error, StringError, WrapError};
