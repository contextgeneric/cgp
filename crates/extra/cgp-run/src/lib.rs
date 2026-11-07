#![no_std]
//! This crate provides CGP's task runners, which run a task named by a type-level code.
//!
//! See [`CanRun`](https://contextgeneric.dev/docs/reference/components/runner) and [`CanSendRun`](https://contextgeneric.dev/docs/reference/components/send_runner).
#![warn(missing_docs)]

use core::future::Future;
use core::marker::PhantomData;

use cgp::prelude::*;

/// The async component that runs a task named at the type level to a Result, with each task code
/// dispatched to its own provider through wiring.
///
/// See [`CanRun`](https://contextgeneric.dev/docs/reference/components/runner).
#[cgp_component(Runner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
#[use_type(HasErrorType.Error)]
pub trait CanRun<Code> {
    /// Runs the task.
    async fn run(&self, _code: PhantomData<Code>) -> Result<(), Error>;
}

/// The runner component whose future is Send, supplied by a proxy on the concrete context so a
/// spawning provider can hand work to a work-stealing executor.
///
/// See [`CanSendRun`](https://contextgeneric.dev/docs/reference/components/send_runner).
#[cgp_component(SendRunner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
#[use_type(HasErrorType.Error)]
pub trait CanSendRun<Code> {
    /// Runs the task and returns a `Send` future.
    fn send_run(&self, _code: PhantomData<Code>) -> impl Future<Output = Result<(), Error>> + Send;
}
