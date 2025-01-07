#[cfg(feature = "alloc")]
mod alloc;

mod discard_detail;
mod infallible;
mod panic_error;
mod raise_from;
mod return_error;

#[cfg(feature = "alloc")]
pub use alloc::*;

pub use discard_detail::*;
pub use infallible::*;
pub use panic_error::*;
pub use raise_from::*;
pub use return_error::*;
