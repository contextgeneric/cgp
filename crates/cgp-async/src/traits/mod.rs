pub mod r#async;
pub mod send;
pub mod r#static;
pub mod sync;

pub use r#async::Async;
pub use send::MaybeSend;
pub use r#static::MaybeStatic;
pub use sync::MaybeSync;
