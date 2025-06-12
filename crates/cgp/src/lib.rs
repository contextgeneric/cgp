#![no_std]
#![doc = include_str!("../README.md")]

#[doc(inline)]
pub use {
    cgp_core as core,
    cgp_core::{prelude, re_export_imports},
    cgp_extra as extra,
};
