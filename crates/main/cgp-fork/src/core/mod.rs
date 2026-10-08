pub mod prelude;

/// Names the macro expansion refers to. A superset of [`prelude`]: the error
/// provider traits live here so generated impls can name them without also
/// putting `ErrorRaiser` next to `CanRaiseError` in the user prelude, which
/// would make `Context::raise_error` ambiguous.
pub mod macro_prelude {
    pub use crate::core::error::{
        ErrorRaiser, ErrorRaiserComponent, ErrorTypeProvider, ErrorTypeProviderComponent,
        ErrorWrapper, ErrorWrapperComponent,
    };
    pub use crate::core::prelude::*;
}

pub mod base;
pub(crate) mod base_types;
pub mod component;
pub mod error;
pub mod field;
pub mod types;

pub use cgp_fork_macro as macros;
pub use cgp_fork_macro::async_trait;
