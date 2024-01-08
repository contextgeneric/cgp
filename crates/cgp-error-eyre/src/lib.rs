use cgp_core::prelude::*;
use cgp_core::ErrorRaiser;
use cgp_core::ProvideErrorType;
use core::fmt::Debug;
use core::fmt::Display;
use eyre::{eyre, Report};
use std::error::Error as StdError;

pub struct ProvideEyreError;

impl<Context> ProvideErrorType<Context> for ProvideEyreError
where
    Context: Async,
{
    type Error = Report;
}

pub struct RaiseStdError;

impl<Context, E> ErrorRaiser<Context, E> for RaiseStdError
where
    Context: HasErrorType<Error = Report>,
    E: StdError + Async,
{
    fn raise_error(e: E) -> Report {
        e.into()
    }
}

pub struct RaiseDebugError;

impl<Context, E> ErrorRaiser<Context, E> for RaiseDebugError
where
    Context: HasErrorType<Error = Report>,
    E: Debug,
{
    fn raise_error(e: E) -> Report {
        eyre!("{:?}", e)
    }
}

pub struct RaiseDisplayError;

impl<Context, E> ErrorRaiser<Context, E> for RaiseDisplayError
where
    Context: HasErrorType<Error = Report>,
    E: Display,
{
    fn raise_error(e: E) -> Report {
        eyre!("{e}")
    }
}
