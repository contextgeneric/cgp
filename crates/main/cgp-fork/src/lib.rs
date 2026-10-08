#![no_std]
#![doc = include_str!("../README.md")]
#![allow(mixed_script_confusables)]

extern crate alloc;
extern crate self as cgp_fork;

pub mod core;
pub mod extra;

pub mod prelude;

/// Names the macro expansion refers to. A superset of [`prelude`], so generated
/// paths such as `::cgp_fork::macro_prelude::ErrorRaiser` resolve without exporting
/// the provider traits from the user prelude.
pub mod macro_prelude {
    pub use crate::core::error::{
        ErrorRaiser, ErrorRaiserComponent, ErrorTypeProvider, ErrorTypeProviderComponent,
        ErrorWrapper, ErrorWrapperComponent,
    };
    pub use crate::prelude::*;
}

#[cfg(feature = "anyhow")]
#[doc = include_str!("anyhow/README.md")]
pub mod anyhow;

#[cfg(feature = "eyre")]
#[doc = include_str!("eyre/README.md")]
pub mod eyre;

#[cfg(feature = "std-error")]
#[doc = include_str!("std_error/README.md")]
pub mod std_error;
