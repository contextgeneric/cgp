use cgp_async::Async;

use crate::traits::HasErrorType;

pub trait HasAsyncErrorType: Async + HasErrorType<Error: Async> {}

impl<Context> HasAsyncErrorType for Context where Context: Async + HasErrorType<Error: Async> {}
