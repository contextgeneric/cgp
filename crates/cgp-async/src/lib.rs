#![no_std]

pub mod traits;

#[cfg(feature = "async")]
pub use cgp_async_macro::native_async as async_trait;
#[cfg(not(feature = "async"))]
pub use cgp_sync::async_trait;
pub use traits::{Async, MaybeSend, MaybeStatic, MaybeSync};
