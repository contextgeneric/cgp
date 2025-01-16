use cgp_async::Async;

use crate::traits::HasErrorType;
use crate::{CanRaiseError, CanWrapError};

pub trait HasAsyncErrorType: Async + HasErrorType<Error: Async> {}

impl<Context> HasAsyncErrorType for Context where Context: Async + HasErrorType<Error: Async> {}

pub trait CanRaiseAsyncError<SourceError>: HasAsyncErrorType + CanRaiseError<SourceError> {}

impl<Context, SourceError> CanRaiseAsyncError<SourceError> for Context where
    Context: HasAsyncErrorType + CanRaiseError<SourceError>
{
}

pub trait CanWrapAsyncError<SourceError>: HasAsyncErrorType + CanWrapError<SourceError> {}

impl<Context, SourceError> CanWrapAsyncError<SourceError> for Context where
    Context: HasAsyncErrorType + CanWrapError<SourceError>
{
}
