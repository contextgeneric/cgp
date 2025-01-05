use cgp_async::Async;

use crate::HasErrorType;

pub trait HasAsyncErrorType: HasErrorType<Error: Async> {}

impl<Context> HasAsyncErrorType for Context where Context: HasErrorType<Error: Async> {}
