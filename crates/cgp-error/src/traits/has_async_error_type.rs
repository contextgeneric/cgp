use cgp_async::Async;

use crate::traits::HasErrorType;

pub trait HasAsyncErrorType: HasErrorType<Error: Async> {}

impl<Context> HasAsyncErrorType for Context where Context: HasErrorType<Error: Async> {}
