#![no_std]

use core::error::Error as StdError;
use core::fmt::{Debug, Display};

use anyhow::{anyhow, Error};
use cgp_core::error::{ErrorRaiser, ProvideErrorType};
use cgp_core::prelude::*;

pub struct UseAnyhowError;

impl<Context> ProvideErrorType<Context> for UseAnyhowError {
    type Error = Error;
}

pub struct RaiseStdError;

impl<Context, E> ErrorRaiser<Context, E> for RaiseStdError
where
    Context: HasErrorType<Error = Error>,
    E: StdError + Send + Sync + 'static,
{
    fn raise_error(e: E) -> Error {
        e.into()
    }
}

pub struct DebugError;

impl<Context, E> ErrorRaiser<Context, E> for DebugError
where
    Context: HasErrorType<Error = Error>,
    E: Debug,
{
    fn raise_error(e: E) -> Error {
        anyhow!("{:?}", e)
    }
}

pub struct DisplayError;

impl<Context, E> ErrorRaiser<Context, E> for DisplayError
where
    Context: HasErrorType<Error = Error>,
    E: Display,
{
    fn raise_error(e: E) -> Error {
        anyhow!("{e}")
    }
}
