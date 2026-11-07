#![no_std]
//! This crate re-exports CGP's extra layer through `cgp::extra`: handlers, monads, dispatch,
//! runtime, runners, the optional-field layer, and the generic error providers. The `alloc` feature
//! turns on the providers that need an allocator.
//!
//! See [`CanHandle`](https://contextgeneric.dev/docs/reference/components/handler/handler), [`PipeHandlers`](https://contextgeneric.dev/docs/reference/providers/handler/pipe_handlers), and [`RaiseFrom`](https://contextgeneric.dev/docs/reference/providers/error/raise_from).
#![warn(missing_docs)]

/// The prelude, re-exported as `cgp_extra::prelude`.
pub mod prelude;

pub use cgp_dispatch as dispatch;
pub use cgp_error_extra as error;
pub use cgp_field_extra as field;
pub use cgp_handler as handler;
pub use cgp_monad as monad;
pub use cgp_run as run;
pub use cgp_runtime as runtime;
