//! `#[cgp_computer]` on an `async` function: producing a provider usable as a
//! `Handler`.
//!
//! An `async` function annotated with `#[cgp_computer]` becomes an
//! `AsyncComputer` that CGP promotes into the async, fallible `Handler`, so the
//! same definition is callable as `compute_async` and `handle` (plus the by-ref
//! `compute_async_ref` / `handle_ref`). A `Result`-returning async body promotes
//! so its error path surfaces through `handle`.
//!
//! The error wiring on `App` is incidental scaffolding, so it uses the plain
//! `delegate_components!`.
//!
//! The async value and `Result` expansions of `#[cgp_computer]` are pinned here.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_computer.md,
//! cgp-knowledge-base/cgp/reference/components/handler.md, and
//! cgp-knowledge-base/cgp/reference/components/computer.md.

use core::fmt::Display;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::RaiseFrom;
use cgp::extra::handler::HandlerRef;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_computer;
use futures::executor::block_on;

snapshot_cgp_computer! {
    #[cgp_computer]
    async fn add(a: u64, b: u64) -> u64 {
        a + b
    }

    expand_async_add(output) {
        insta::assert_snapshot!(output, @"
        async fn add(a: u64, b: u64) -> u64 {
            a + b
        }
        impl<__Context__, __Code__> AsyncComputer<__Context__, __Code__, (u64, u64)> for Add {
            type Output = u64;
            async fn compute_async(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0, arg_1): (u64, u64),
            ) -> Self::Output {
                add(arg_0, arg_1).await
            }
        }
        impl<
            __Context__,
            __Code__,
        > IsProviderFor<AsyncComputerComponent, __Context__, (__Code__, (u64, u64))> for Add {}
        pub struct Add;
        impl DelegateComponent<AsyncComputerRefComponent> for Add
        where
            PromoteAsyncComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
        {
            type Delegate = <PromoteAsyncComputer<
                Self,
            > as DelegateComponent<AsyncComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for Add
        where
            PromoteAsyncComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
            <PromoteAsyncComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerRefComponent,
            >>::Delegate: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for Add
        where
            PromoteAsyncComputer<Self>: DelegateComponent<HandlerComponent>,
        {
            type Delegate = <PromoteAsyncComputer<
                Self,
            > as DelegateComponent<HandlerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for Add
        where
            PromoteAsyncComputer<Self>: DelegateComponent<HandlerComponent>,
            <PromoteAsyncComputer<
                Self,
            > as DelegateComponent<
                HandlerComponent,
            >>::Delegate: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for Add
        where
            PromoteAsyncComputer<Self>: DelegateComponent<HandlerRefComponent>,
        {
            type Delegate = <PromoteAsyncComputer<
                Self,
            > as DelegateComponent<HandlerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for Add
        where
            PromoteAsyncComputer<Self>: DelegateComponent<HandlerRefComponent>,
            <PromoteAsyncComputer<
                Self,
            > as DelegateComponent<
                HandlerRefComponent,
            >>::Delegate: IsProviderFor<HandlerRefComponent, __Context__, __Params__>,
        {}
        ")
    }
}

snapshot_cgp_computer! {
    #[cgp_computer]
    async fn add_with_error(a: u64, b: u64) -> Result<u64, String> {
        a.checked_add(b).ok_or_else(|| "Overflow".to_string())
    }

    expand_async_add_with_error(output) {
        insta::assert_snapshot!(output, @r#"
        async fn add_with_error(a: u64, b: u64) -> Result<u64, String> {
            a.checked_add(b).ok_or_else(|| "Overflow".to_string())
        }
        impl<__Context__, __Code__> AsyncComputer<__Context__, __Code__, (u64, u64)>
        for AddWithError {
            type Output = Result<u64, String>;
            async fn compute_async(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0, arg_1): (u64, u64),
            ) -> Self::Output {
                add_with_error(arg_0, arg_1).await
            }
        }
        impl<
            __Context__,
            __Code__,
        > IsProviderFor<AsyncComputerComponent, __Context__, (__Code__, (u64, u64))>
        for AddWithError {}
        pub struct AddWithError;
        impl DelegateComponent<AsyncComputerRefComponent> for AddWithError
        where
            PromoteHandler<Self>: DelegateComponent<AsyncComputerRefComponent>,
        {
            type Delegate = <PromoteHandler<
                Self,
            > as DelegateComponent<AsyncComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for AddWithError
        where
            PromoteHandler<Self>: DelegateComponent<AsyncComputerRefComponent>,
            <PromoteHandler<
                Self,
            > as DelegateComponent<
                AsyncComputerRefComponent,
            >>::Delegate: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for AddWithError
        where
            PromoteHandler<Self>: DelegateComponent<HandlerComponent>,
        {
            type Delegate = <PromoteHandler<
                Self,
            > as DelegateComponent<HandlerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for AddWithError
        where
            PromoteHandler<Self>: DelegateComponent<HandlerComponent>,
            <PromoteHandler<
                Self,
            > as DelegateComponent<
                HandlerComponent,
            >>::Delegate: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for AddWithError
        where
            PromoteHandler<Self>: DelegateComponent<HandlerRefComponent>,
        {
            type Delegate = <PromoteHandler<
                Self,
            > as DelegateComponent<HandlerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for AddWithError
        where
            PromoteHandler<Self>: DelegateComponent<HandlerRefComponent>,
            <PromoteHandler<
                Self,
            > as DelegateComponent<
                HandlerRefComponent,
            >>::Delegate: IsProviderFor<HandlerRefComponent, __Context__, __Params__>,
        {}
        "#)
    }
}

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent:
            UseType<String>,
        ErrorRaiserComponent:
            RaiseFrom,
    }
}

#[test]
fn test_add() {
    let app = App;

    assert_eq!(
        block_on(Add::compute_async(&app, PhantomData::<()>, (1, 2))),
        3,
    );

    assert_eq!(
        block_on(Add::handle(&app, PhantomData::<()>, (1, 2))),
        Ok(3),
    );
}

#[test]
fn test_add_with_error() {
    let app = App;

    assert_eq!(
        block_on(AddWithError::compute_async(&app, PhantomData::<()>, (1, 2))),
        Ok(3),
    );

    assert_eq!(
        block_on(AddWithError::handle(&app, PhantomData::<()>, (1, 2))),
        Ok(3),
    );

    assert_eq!(
        block_on(AddWithError::handle(&app, PhantomData::<()>, (u64::MAX, 1))),
        Err("Overflow".to_string()),
    );
}

#[cgp_computer]
async fn to_string_ref<Value: Display + Sync>(value: &Value) -> String {
    value.to_string()
}

#[test]
fn test_handler_ref() {
    let app = App;
    let code = PhantomData::<()>;

    assert_eq!(block_on(ToStringRef::compute_async(&app, code, &1)), "1");

    assert_eq!(
        block_on(ToStringRef::compute_async_ref(&app, code, &1)),
        "1"
    );

    assert_eq!(
        block_on(ToStringRef::handle(&app, code, &1)),
        Ok("1".to_owned())
    );

    assert_eq!(
        block_on(ToStringRef::handle_ref(&app, code, &1)),
        Ok("1".to_owned())
    );
}
