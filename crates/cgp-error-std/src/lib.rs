#![no_std]

extern crate alloc;

mod impls;
mod types;

pub use impls::{DebugBoxedStdError, DisplayBoxedStdError, RaiseBoxedStdError, UseBoxedStdError};
pub use types::{Error, StringError};
