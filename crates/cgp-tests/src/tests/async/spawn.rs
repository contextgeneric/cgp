/**
   This test demonstrates how something like tokio::spawn can be used without
   us needing to annotate Future: Send everywhere in our code.

   The key is to implement a proxy SendRunner on the concrete context, which
   would verify that all abstract types are safe to be sent in the produced
   futures.

   This workaround is needed until the Return Type Notation (RTN) feature in
   Rust is stabilized.
*/
use core::convert::Infallible;
use core::future::Future;

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::extra::run::{
    CanRun, CanSendRun, Runner, RunnerComponent, SendRunner, SendRunnerComponent,
};
use cgp::prelude::*;
use futures::executor::block_on;

// A dummy spawn function that has the same signature as tokio::spawn,
// requiring the Future to implement Send + 'static.
fn dummy_spawn<F>(_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
}

// The abstract types and interfaces do not contain explicit Send bounds

#[cgp_type]
pub trait HasFooType {
    type Foo;
}

#[cgp_type]
pub trait HasBarType {
    type Bar;
}

#[cgp_component(FooFetcher)]
#[async_trait]
pub trait CanFetchFoo: HasFooType + HasErrorType {
    async fn fetch_foo(&self) -> Result<Self::Foo, Self::Error>;
}

#[cgp_component(BarFetcher)]
#[async_trait]
pub trait CanFetchBar: HasBarType + HasErrorType {
    async fn fetch_bar(&self) -> Result<Self::Bar, Self::Error>;
}

#[cgp_component(FooBarRunner)]
#[async_trait]
pub trait CanRunFooBar: HasFooType + HasBarType + HasErrorType {
    async fn run_foo_bar(&self, foo: &Self::Foo, bar: &Self::Bar) -> Result<(), Self::Error>;
}

// Abstract providers can be implemented without Send bounds

#[cgp_new_provider(RunnerComponent)]
impl<Context, Code> Runner<Context, Code> for RunWithFooBar
where
    Context: CanFetchFoo + CanFetchBar + CanRunFooBar,
{
    async fn run(context: &Context, _code: PhantomData<Code>) -> Result<(), Context::Error> {
        let foo = context.fetch_foo().await?;
        let bar = context.fetch_bar().await?;

        context.run_foo_bar(&foo, &bar).await?;

        Ok(())
    }
}

#[cgp_new_provider(RunnerComponent)]
impl<Context, Code, InCode> Runner<Context, Code> for SpawnAndRun<InCode>
where
    Context: 'static + Send + Clone + CanSendRun<InCode>,
{
    async fn run(context: &Context, _code: PhantomData<Code>) -> Result<(), Context::Error> {
        let context = context.clone();

        dummy_spawn(async move {
            let _ = context.send_run(PhantomData).await;
        });

        Ok(())
    }
}

#[cgp_new_provider]
impl<Context> FooFetcher<Context> for DummyFetchFoo
where
    Context: HasFooType<Foo: Default> + HasErrorType,
{
    async fn fetch_foo(_context: &Context) -> Result<Context::Foo, Context::Error> {
        Ok(Default::default())
    }
}

#[cgp_new_provider]
impl<Context> BarFetcher<Context> for DummyFetchBar
where
    Context: HasBarType<Bar: Default> + HasErrorType,
{
    async fn fetch_bar(_context: &Context) -> Result<Context::Bar, Context::Error> {
        Ok(Default::default())
    }
}

#[cgp_new_provider]
impl<Context> FooBarRunner<Context> for DummyRunFoobar
where
    Context: HasFooType + HasBarType + HasErrorType,
{
    async fn run_foo_bar(
        _context: &Context,
        _foo: &Context::Foo,
        _bar: &Context::Bar,
    ) -> Result<(), Context::Error> {
        Ok(())
    }
}

// An example App context that has Send-safe implementationsS

#[cgp_context]
#[derive(Clone)]
pub struct App;

pub struct ActionA;
pub struct ActionB;

delegate_components! {
    AppComponents {
        ErrorTypeProviderComponent:
            UseType<Infallible>,
        [
            FooTypeProviderComponent,
            BarTypeProviderComponent,
        ]:
            UseType<()>,
        FooFetcherComponent:
            DummyFetchFoo,
        BarFetcherComponent:
            DummyFetchBar,
        FooBarRunnerComponent:
            DummyRunFoobar,
        RunnerComponent:
            UseDelegate<new AppRunnerComponents {
                ActionA: RunWithFooBar,
                ActionB: SpawnAndRun<ActionA>,
            }>,
    }
}

// Explicit implementation of SendRunner for App, by forwarding the
// call to Runner that is implemented by `RunWithFooBar`.
// With the concrete context known, the Send bound can be found in the concrete future.

#[cgp_provider]
impl SendRunner<App, ActionA> for AppComponents {
    async fn send_run(context: &App, code: PhantomData<ActionA>) -> Result<(), Infallible> {
        context.run(code).await
    }
}

#[test]
pub fn test_async_spawn() {
    let app = App;
    block_on(app.run(PhantomData::<ActionB>)).unwrap();
}
