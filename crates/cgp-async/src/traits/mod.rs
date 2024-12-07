pub mod r#async;
pub mod send;
pub mod r#static;
pub mod sync;

pub use r#async::Async;
pub use r#static::MaybeStatic;
pub use send::MaybeSend;
pub use sync::MaybeSync;
