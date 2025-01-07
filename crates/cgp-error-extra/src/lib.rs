#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;

pub use impls::*;
