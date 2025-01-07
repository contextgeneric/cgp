#[cfg(feature = "alloc")]
mod alloc;

mod discard_detail;
mod infallible;
mod panic_error;
mod raise_from;
mod return_error;

#[cfg(feature = "alloc")]
pub use alloc::{DebugError, DisplayError};

pub use discard_detail::DiscardDetail;
pub use infallible::RaiseInfallible;
pub use panic_error::PanicOnError;
pub use raise_from::RaiseFrom;
pub use return_error::ReturnError;
