#![no_std]
#![allow(mixed_script_confusables)]

pub mod prelude;

/// Names the macro expansion refers to. A superset of [`prelude`]: the error
/// provider traits live here so generated impls can name them without also
/// putting `ErrorRaiser` next to `CanRaiseError` in the user prelude, which
/// would make `Context::raise_error` ambiguous.
pub mod macro_prelude {
    pub use crate::error::{
        ErrorRaiser, ErrorRaiserComponent, ErrorTypeProvider, ErrorTypeProviderComponent,
        ErrorWrapper, ErrorWrapperComponent,
    };
    pub use crate::prelude::*;
}
#[doc(inline)]
pub use {
    cgp_async_macro::async_trait, cgp_base as base, cgp_component as component, cgp_error as error,
    cgp_field as field, cgp_macro as macros, cgp_type as types,
};
