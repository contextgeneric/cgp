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
use cgp_macro_test_util::{snapshot_cgp_component, snapshot_delegate_components};
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

snapshot_cgp_component! {
    #[cgp_component(FooFetcher)]
    #[async_trait]
    pub trait CanFetchFoo: HasFooType + HasErrorType {
        async fn fetch_foo(&self) -> Result<Self::Foo, Self::Error>;
    }

    expand_can_fetch_foo(output) {
        insta::assert_snapshot!(output, @"
        #[async_trait]
        pub trait CanFetchFoo: HasFooType + HasErrorType {
            async fn fetch_foo(&self) -> Result<Self::Foo, Self::Error>;
        }
        #[async_trait]
        impl<__Context__> CanFetchFoo for __Context__
        where
            __Context__: HasFooType + HasErrorType,
            __Context__: FooFetcher<__Context__>,
        {
            async fn fetch_foo(&self) -> Result<Self::Foo, Self::Error> {
                __Context__::fetch_foo(self).await
            }
        }
        #[async_trait]
        pub trait FooFetcher<__Context__>: IsProviderFor<FooFetcherComponent, __Context__, ()>
        where
            __Context__: HasFooType + HasErrorType,
        {
            async fn fetch_foo(
                __context__: &__Context__,
            ) -> Result<__Context__::Foo, __Context__::Error>;
        }
        #[async_trait]
        impl<__Provider__, __Context__> FooFetcher<__Context__> for __Provider__
        where
            __Context__: HasFooType + HasErrorType,
            __Provider__: DelegateComponent<FooFetcherComponent>
                + IsProviderFor<FooFetcherComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                FooFetcherComponent,
            >>::Delegate: FooFetcher<__Context__>,
        {
            async fn fetch_foo(
                __context__: &__Context__,
            ) -> Result<__Context__::Foo, __Context__::Error> {
                <__Provider__ as DelegateComponent<
                    FooFetcherComponent,
                >>::Delegate::fetch_foo(__context__)
                    .await
            }
        }
        pub struct FooFetcherComponent;
        #[async_trait]
        impl<__Context__> FooFetcher<__Context__> for UseContext
        where
            __Context__: HasFooType + HasErrorType,
            __Context__: CanFetchFoo,
        {
            async fn fetch_foo(
                __context__: &__Context__,
            ) -> Result<__Context__::Foo, __Context__::Error> {
                __Context__::fetch_foo(__context__).await
            }
        }
        impl<__Context__> IsProviderFor<FooFetcherComponent, __Context__, ()> for UseContext
        where
            __Context__: HasFooType + HasErrorType,
            __Context__: CanFetchFoo,
        {}
        #[async_trait]
        impl<__Context__, __Components__, __Path__> FooFetcher<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooType + HasErrorType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: FooFetcher<__Context__>,
        {
            async fn fetch_foo(
                __context__: &__Context__,
            ) -> Result<__Context__::Foo, __Context__::Error> {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::fetch_foo(__context__)
                    .await
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<FooFetcherComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooType + HasErrorType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<FooFetcherComponent, __Context__, ()>
                + FooFetcher<__Context__>,
        {}
        ")
    }
}

snapshot_cgp_component! {
    #[cgp_component(BarFetcher)]
    #[async_trait]
    pub trait CanFetchBar: HasBarType + HasErrorType {
        async fn fetch_bar(&self) -> Result<Self::Bar, Self::Error>;
    }

    expand_can_fetch_bar(output) {
        insta::assert_snapshot!(output, @"
        #[async_trait]
        pub trait CanFetchBar: HasBarType + HasErrorType {
            async fn fetch_bar(&self) -> Result<Self::Bar, Self::Error>;
        }
        #[async_trait]
        impl<__Context__> CanFetchBar for __Context__
        where
            __Context__: HasBarType + HasErrorType,
            __Context__: BarFetcher<__Context__>,
        {
            async fn fetch_bar(&self) -> Result<Self::Bar, Self::Error> {
                __Context__::fetch_bar(self).await
            }
        }
        #[async_trait]
        pub trait BarFetcher<__Context__>: IsProviderFor<BarFetcherComponent, __Context__, ()>
        where
            __Context__: HasBarType + HasErrorType,
        {
            async fn fetch_bar(
                __context__: &__Context__,
            ) -> Result<__Context__::Bar, __Context__::Error>;
        }
        #[async_trait]
        impl<__Provider__, __Context__> BarFetcher<__Context__> for __Provider__
        where
            __Context__: HasBarType + HasErrorType,
            __Provider__: DelegateComponent<BarFetcherComponent>
                + IsProviderFor<BarFetcherComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                BarFetcherComponent,
            >>::Delegate: BarFetcher<__Context__>,
        {
            async fn fetch_bar(
                __context__: &__Context__,
            ) -> Result<__Context__::Bar, __Context__::Error> {
                <__Provider__ as DelegateComponent<
                    BarFetcherComponent,
                >>::Delegate::fetch_bar(__context__)
                    .await
            }
        }
        pub struct BarFetcherComponent;
        #[async_trait]
        impl<__Context__> BarFetcher<__Context__> for UseContext
        where
            __Context__: HasBarType + HasErrorType,
            __Context__: CanFetchBar,
        {
            async fn fetch_bar(
                __context__: &__Context__,
            ) -> Result<__Context__::Bar, __Context__::Error> {
                __Context__::fetch_bar(__context__).await
            }
        }
        impl<__Context__> IsProviderFor<BarFetcherComponent, __Context__, ()> for UseContext
        where
            __Context__: HasBarType + HasErrorType,
            __Context__: CanFetchBar,
        {}
        #[async_trait]
        impl<__Context__, __Components__, __Path__> BarFetcher<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasBarType + HasErrorType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: BarFetcher<__Context__>,
        {
            async fn fetch_bar(
                __context__: &__Context__,
            ) -> Result<__Context__::Bar, __Context__::Error> {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::fetch_bar(__context__)
                    .await
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<BarFetcherComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasBarType + HasErrorType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<BarFetcherComponent, __Context__, ()>
                + BarFetcher<__Context__>,
        {}
        ")
    }
}

snapshot_cgp_component! {
    #[cgp_component(FooBarRunner)]
    #[async_trait]
    pub trait CanRunFooBar: HasFooType + HasBarType + HasErrorType {
        async fn run_foo_bar(&self, foo: &Self::Foo, bar: &Self::Bar) -> Result<(), Self::Error>;
    }

    expand_can_run_foo_bar(output) {
        insta::assert_snapshot!(output, @"
        #[async_trait]
        pub trait CanRunFooBar: HasFooType + HasBarType + HasErrorType {
            async fn run_foo_bar(
                &self,
                foo: &Self::Foo,
                bar: &Self::Bar,
            ) -> Result<(), Self::Error>;
        }
        #[async_trait]
        impl<__Context__> CanRunFooBar for __Context__
        where
            __Context__: HasFooType + HasBarType + HasErrorType,
            __Context__: FooBarRunner<__Context__>,
        {
            async fn run_foo_bar(
                &self,
                foo: &Self::Foo,
                bar: &Self::Bar,
            ) -> Result<(), Self::Error> {
                __Context__::run_foo_bar(self, foo, bar).await
            }
        }
        #[async_trait]
        pub trait FooBarRunner<
            __Context__,
        >: IsProviderFor<FooBarRunnerComponent, __Context__, ()>
        where
            __Context__: HasFooType + HasBarType + HasErrorType,
        {
            async fn run_foo_bar(
                __context__: &__Context__,
                foo: &__Context__::Foo,
                bar: &__Context__::Bar,
            ) -> Result<(), __Context__::Error>;
        }
        #[async_trait]
        impl<__Provider__, __Context__> FooBarRunner<__Context__> for __Provider__
        where
            __Context__: HasFooType + HasBarType + HasErrorType,
            __Provider__: DelegateComponent<FooBarRunnerComponent>
                + IsProviderFor<FooBarRunnerComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                FooBarRunnerComponent,
            >>::Delegate: FooBarRunner<__Context__>,
        {
            async fn run_foo_bar(
                __context__: &__Context__,
                foo: &__Context__::Foo,
                bar: &__Context__::Bar,
            ) -> Result<(), __Context__::Error> {
                <__Provider__ as DelegateComponent<
                    FooBarRunnerComponent,
                >>::Delegate::run_foo_bar(__context__, foo, bar)
                    .await
            }
        }
        pub struct FooBarRunnerComponent;
        #[async_trait]
        impl<__Context__> FooBarRunner<__Context__> for UseContext
        where
            __Context__: HasFooType + HasBarType + HasErrorType,
            __Context__: CanRunFooBar,
        {
            async fn run_foo_bar(
                __context__: &__Context__,
                foo: &__Context__::Foo,
                bar: &__Context__::Bar,
            ) -> Result<(), __Context__::Error> {
                __Context__::run_foo_bar(__context__, foo, bar).await
            }
        }
        impl<__Context__> IsProviderFor<FooBarRunnerComponent, __Context__, ()> for UseContext
        where
            __Context__: HasFooType + HasBarType + HasErrorType,
            __Context__: CanRunFooBar,
        {}
        #[async_trait]
        impl<__Context__, __Components__, __Path__> FooBarRunner<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooType + HasBarType + HasErrorType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: FooBarRunner<__Context__>,
        {
            async fn run_foo_bar(
                __context__: &__Context__,
                foo: &__Context__::Foo,
                bar: &__Context__::Bar,
            ) -> Result<(), __Context__::Error> {
                <__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate::run_foo_bar(__context__, foo, bar)
                    .await
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<FooBarRunnerComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooType + HasBarType + HasErrorType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<FooBarRunnerComponent, __Context__, ()>
                + FooBarRunner<__Context__>,
        {}
        ")
    }
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

#[derive(Clone)]
pub struct App;

pub struct ActionA;
pub struct ActionB;

snapshot_delegate_components! {
    delegate_components! {
        App {
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

    expand_app(output) {
        insta::assert_snapshot!(output, @"
        pub struct AppRunnerComponents;
        impl DelegateComponent<ErrorTypeProviderComponent> for App {
            type Delegate = UseType<Infallible>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<ErrorTypeProviderComponent, __Context__, __Params__> for App
        where
            UseType<
                Infallible,
            >: IsProviderFor<ErrorTypeProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<FooTypeProviderComponent> for App {
            type Delegate = UseType<()>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<FooTypeProviderComponent, __Context__, __Params__> for App
        where
            UseType<()>: IsProviderFor<FooTypeProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<BarTypeProviderComponent> for App {
            type Delegate = UseType<()>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<BarTypeProviderComponent, __Context__, __Params__> for App
        where
            UseType<()>: IsProviderFor<BarTypeProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<FooFetcherComponent> for App {
            type Delegate = DummyFetchFoo;
        }
        impl<__Context__, __Params__> IsProviderFor<FooFetcherComponent, __Context__, __Params__>
        for App
        where
            DummyFetchFoo: IsProviderFor<FooFetcherComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<BarFetcherComponent> for App {
            type Delegate = DummyFetchBar;
        }
        impl<__Context__, __Params__> IsProviderFor<BarFetcherComponent, __Context__, __Params__>
        for App
        where
            DummyFetchBar: IsProviderFor<BarFetcherComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<FooBarRunnerComponent> for App {
            type Delegate = DummyRunFoobar;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<FooBarRunnerComponent, __Context__, __Params__> for App
        where
            DummyRunFoobar: IsProviderFor<FooBarRunnerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<RunnerComponent> for App {
            type Delegate = UseDelegate<AppRunnerComponents>;
        }
        impl<__Context__, __Params__> IsProviderFor<RunnerComponent, __Context__, __Params__>
        for App
        where
            UseDelegate<
                AppRunnerComponents,
            >: IsProviderFor<RunnerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<ActionA> for AppRunnerComponents {
            type Delegate = RunWithFooBar;
        }
        impl<__Context__, __Params__> IsProviderFor<ActionA, __Context__, __Params__>
        for AppRunnerComponents
        where
            RunWithFooBar: IsProviderFor<ActionA, __Context__, __Params__>,
        {}
        impl DelegateComponent<ActionB> for AppRunnerComponents {
            type Delegate = SpawnAndRun<ActionA>;
        }
        impl<__Context__, __Params__> IsProviderFor<ActionB, __Context__, __Params__>
        for AppRunnerComponents
        where
            SpawnAndRun<ActionA>: IsProviderFor<ActionB, __Context__, __Params__>,
        {}
        ")
    }
}

// Explicit implementation of SendRunner for App, by forwarding the
// call to Runner that is implemented by `RunWithFooBar`.
// With the concrete context known, the Send bound can be found in the concrete future.

#[cgp_provider]
impl SendRunner<App, ActionA> for App {
    async fn send_run(context: &App, code: PhantomData<ActionA>) -> Result<(), Infallible> {
        context.run(code).await
    }
}

#[test]
pub fn test_async_spawn() {
    let app = App;
    block_on(app.run(PhantomData::<ActionB>)).unwrap();
}
