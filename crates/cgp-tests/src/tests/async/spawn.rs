use core::future::Future;

use cgp::extra::run::{Runner, RunnerComponent};
use cgp::prelude::*;

#[test]
pub fn test_async_spawn() {
    fn dummy_spawn<F>(_future: F)
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
    }

    #[cgp_type]
    pub trait HasFooType {
        type Foo: Async;
    }

    #[cgp_type]
    pub trait HasBarType {
        type Bar: Async;
    }

    #[cgp_component {
        provider: FooFetcher,
    }]
    #[async_trait]
    pub trait CanFetchFoo: Async + HasFooType + HasAsyncErrorType {
        async fn fetch_foo(&self) -> Result<Self::Foo, Self::Error>;
    }

    #[cgp_component {
        provider: BarFetcher,
    }]
    #[async_trait]
    pub trait CanFetchBar: Async + HasBarType + HasAsyncErrorType {
        async fn fetch_bar(&self) -> Result<Self::Bar, Self::Error>;
    }

    #[cgp_component {
        provider: FooBarRunner,
    }]
    #[async_trait]
    pub trait CanRunFooBar: Async + HasFooType + HasBarType + HasAsyncErrorType {
        async fn run_foo_bar(&self, foo: &Self::Foo, bar: &Self::Bar) -> Result<(), Self::Error>;
    }

    #[cgp_new_provider(RunnerComponent)]
    impl<Context> Runner<Context> for RunWithFooBar
    where
        Context: Async + CanFetchFoo + CanFetchBar + CanRunFooBar,
    {
        async fn run(context: &Context) -> Result<(), Context::Error> {
            let foo = context.fetch_foo().await?;
            let bar = context.fetch_bar().await?;

            context.run_foo_bar(&foo, &bar).await?;

            Ok(())
        }
    }

    #[cgp_new_provider(RunnerComponent)]
    impl<Context> Runner<Context> for SpawnAndRunWithFooBar
    where
        Context: 'static + Clone + HasAsyncErrorType,
        RunWithFooBar: Runner<Context>,
    {
        async fn run(context: &Context) -> Result<(), Context::Error> {
            let context = context.clone();

            dummy_spawn(async move {
                let _ = RunWithFooBar::run(&context).await;
            });

            Ok(())
        }
    }
}
