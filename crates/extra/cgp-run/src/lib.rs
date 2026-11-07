#![no_std]
//! This crate provides CGP's task runners, which run a task named by a type-level code.
//!
//! See [`CanRun`](https://contextgeneric.dev/docs/reference/components/runner) and [`CanSendRun`](https://contextgeneric.dev/docs/reference/components/send_runner).

use core::future::Future;
use core::marker::PhantomData;

use cgp::prelude::*;

#[cgp_component(Runner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
#[use_type(HasErrorType.Error)]
pub trait CanRun<Code> {
    async fn run(&self, _code: PhantomData<Code>) -> Result<(), Error>;
}

#[cgp_component(SendRunner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
#[use_type(HasErrorType.Error)]
pub trait CanSendRun<Code> {
    fn send_run(&self, _code: PhantomData<Code>) -> impl Future<Output = Result<(), Error>> + Send;
}
