#![no_std]

pub use cgp_async_macro::strip_async as async_trait;

pub trait Async {}

impl<T> Async for T {}
