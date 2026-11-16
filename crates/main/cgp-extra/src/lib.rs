#![doc = include_str!("../README.md")]
#![no_std]
#![warn(missing_docs)]

/// The extra names users write, re-exported through `cgp::prelude`.
pub mod prelude;

/// Every extra name the macros' generated code refers to, re-exported through `cgp::macro_prelude`.
/// Only the macros use this module; import from `cgp::prelude` or the defining module instead.
pub mod macro_prelude;

pub use cgp_dispatch as dispatch;
pub use cgp_error_extra as error;
pub use cgp_field_extra as field;
pub use cgp_handler as handler;
pub use cgp_monad as monad;
pub use cgp_run as run;
pub use cgp_runtime as runtime;
