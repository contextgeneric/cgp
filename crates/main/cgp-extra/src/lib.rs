#![no_std]
//! This crate re-exports CGP's extra layer through `cgp::extra`: handlers, monads, dispatch,
//! runtime, runners, the optional-field layer, and the generic error providers. The `alloc` feature
//! turns on the providers that need an allocator.
//!
//! See [`CanHandle`](https://contextgeneric.dev/docs/reference/components/handler/handler), [`PipeHandlers`](https://contextgeneric.dev/docs/reference/providers/handler/pipe_handlers), and [`RaiseFrom`](https://contextgeneric.dev/docs/reference/providers/error/raise_from).
#![warn(missing_docs)]

/// The extra names users write, re-exported through `cgp::prelude`.
pub mod prelude;

/// Every extra name the macros' generated code refers to, re-exported through `cgp::macro_prelude`.
pub mod macro_prelude;

pub use cgp_dispatch as dispatch;
pub use cgp_error_extra as error;
pub use cgp_field_extra as field;
pub use cgp_handler as handler;
pub use cgp_monad as monad;
pub use cgp_run as run;
pub use cgp_runtime as runtime;
