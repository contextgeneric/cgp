pub mod traits;

pub use traits::{Async, MaybeSend, MaybeStatic, MaybeSync};

#[cfg(feature = "async")]
pub use cgp_async_macro::native_async as async_trait;

#[cfg(not(feature = "async"))]
pub use cgp_sync::async_trait;
