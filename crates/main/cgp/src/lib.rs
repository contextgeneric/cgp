#![no_std]
#![doc = include_str!("../README.md")]

#[doc(inline)]
pub use {cgp_core as core, cgp_extra as extra};

pub mod prelude;

/// Names the macro expansion refers to. A superset of [`prelude`], so generated
/// paths such as `::cgp::macro_prelude::ErrorRaiser` resolve without exporting
/// the provider traits from the user prelude.
pub mod macro_prelude {
    pub use crate::core::error::{
        ErrorRaiser, ErrorRaiserComponent, ErrorTypeProvider, ErrorTypeProviderComponent,
        ErrorWrapper, ErrorWrapperComponent,
    };
    pub use crate::prelude::*;
}
