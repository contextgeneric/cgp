use cgp_async::Async;
use cgp_component_macro::trait_alias;

use crate::traits::HasErrorType;
use crate::{CanRaiseError, CanWrapError};

#[trait_alias]
pub trait HasAsyncErrorType: Async + HasErrorType<Error: Async> {}

#[trait_alias]
pub trait CanRaiseAsyncError<SourceError>: HasAsyncErrorType + CanRaiseError<SourceError> {}

#[trait_alias]
pub trait CanWrapAsyncError<SourceError>: HasAsyncErrorType + CanWrapError<SourceError> {}
