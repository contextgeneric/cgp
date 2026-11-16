#![doc = include_str!("../README.md")]
#![no_std]
#![allow(clippy::needless_lifetimes)]
#![warn(missing_docs)]

mod providers;

pub use providers::*;
