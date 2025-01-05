use cgp_core::Async;

use crate::HasRuntimeType;

pub trait HasAsyncRuntimeType: Async + HasRuntimeType<Runtime: Async> {}

impl<Context> HasAsyncRuntimeType for Context where Context: Async + HasRuntimeType<Runtime: Async> {}
