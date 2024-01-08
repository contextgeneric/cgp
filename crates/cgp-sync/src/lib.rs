pub use cgp_async_macro::strip_async as async_trait;

pub trait Async: Sized + 'static {}

impl<T> Async for T where T: Sized + 'static {}
