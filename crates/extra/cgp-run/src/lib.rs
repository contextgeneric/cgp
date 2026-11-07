#![no_std]
//! This crate provides CGP's task runners, which run a task named by a type-level code.
//!
//! See [`CanRun`](https://contextgeneric.dev/docs/reference/components/runner) and [`CanSendRun`](https://contextgeneric.dev/docs/reference/components/send_runner).
#![warn(missing_docs)]

use core::future::Future;
use core::marker::PhantomData;

use cgp::prelude::*;

#[cgp_component(Runner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
#[use_type(HasErrorType.Error)]
/// Runs a task named by a type-level code.
///
/// See [`CanRun`](https://contextgeneric.dev/docs/reference/components/runner).
pub trait CanRun<Code> {
    /// Runs the task.
    async fn run(&self, _code: PhantomData<Code>) -> Result<(), Error>;
}

#[cgp_component(SendRunner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
#[use_type(HasErrorType.Error)]
/// Runs a task whose future is `Send`.
///
/// See [`CanSendRun`](https://contextgeneric.dev/docs/reference/components/send_runner).
pub trait CanSendRun<Code> {
    /// Runs the task and returns a `Send` future.
    fn send_run(&self, _code: PhantomData<Code>) -> impl Future<Output = Result<(), Error>> + Send;
}
