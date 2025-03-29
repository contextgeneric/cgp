use cgp_async::Async;
use cgp_macro::blanket_trait;

use crate::traits::HasErrorType;
use crate::{CanRaiseError, CanWrapError};

#[blanket_trait]
pub trait HasAsyncErrorType: Async + HasErrorType<Error: Async> {}

#[blanket_trait]
pub trait CanRaiseAsyncError<SourceError>: HasAsyncErrorType + CanRaiseError<SourceError> {}

#[blanket_trait]
pub trait CanWrapAsyncError<SourceError>: HasAsyncErrorType + CanWrapError<SourceError> {}
