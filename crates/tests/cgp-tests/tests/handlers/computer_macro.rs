//! `#[cgp_computer]`: turning a plain function into a `Computer` provider.
//!
//! A synchronous, infallible function annotated with `#[cgp_computer]` becomes a
//! provider usable across the whole computation family — the same definition is
//! callable as `compute`, `try_compute`, `compute_async`, and `handle` (and the
//! by-reference `compute_ref` / `try_compute_ref` / `compute_async_ref` /
//! `handle_ref` variants), because CGP automatically promotes a `Computer` into
//! the more capable `TryComputer`, `AsyncComputer`, and `Handler`. A fallible
//! `Result`-returning body promotes so that its error path surfaces through
//! `try_compute` / `handle`. A generic function parameter carries through to the
//! generated provider.
//!
//! The error wiring on `App` is incidental scaffolding needed so the promoted
//! `Handler` has an error type; it uses the plain `delegate_components!` (the
//! error and wiring macros are owned by other concept targets).
//!
//! This target owns the `#[cgp_computer]` snapshots: the synchronous value and
//! `Result` cases and a generic function are pinned here.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_computer.md and
//! cgp-knowledge-base/cgp/reference/components/computer.md.

use core::fmt::Display;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::RaiseFrom;
use cgp::extra::handler::{ComputerRef, HandlerRef, TryComputerRef};
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_computer;
use futures::executor::block_on;

snapshot_cgp_computer! {
    #[cgp_computer]
    fn add(a: u64, b: u64) -> u64 {
        a + b
    }

    expand_add(output) {
        insta::assert_snapshot!(output, @"
        fn add(a: u64, b: u64) -> u64 {
            a + b
        }
        impl<__Context__, __Code__> Computer<__Context__, __Code__, (u64, u64)> for Add {
            type Output = u64;
            fn compute(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0, arg_1): (u64, u64),
            ) -> Self::Output {
                add(arg_0, arg_1)
            }
        }
        impl<
            __Context__,
            __Code__,
        > IsProviderFor<ComputerComponent, __Context__, (__Code__, (u64, u64))> for Add {}
        pub struct Add;
        impl DelegateComponent<ComputerRefComponent> for Add
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<ComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<ComputerRefComponent, __Context__, __Params__> for Add
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                ComputerRefComponent,
            >>::Delegate: IsProviderFor<ComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerComponent> for Add
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<TryComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerComponent, __Context__, __Params__> for Add
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerComponent,
            >>::Delegate: IsProviderFor<TryComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerRefComponent> for Add
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<TryComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerRefComponent, __Context__, __Params__> for Add
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerRefComponent,
            >>::Delegate: IsProviderFor<TryComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerComponent> for Add
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<AsyncComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerComponent, __Context__, __Params__> for Add
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerComponent,
            >>::Delegate: IsProviderFor<AsyncComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerRefComponent> for Add
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<AsyncComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for Add
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerRefComponent,
            >>::Delegate: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for Add
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<HandlerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for Add
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerComponent,
            >>::Delegate: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for Add
        where
            PromoteComputer<Self>: DelegateComponent<HandlerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<HandlerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for Add
        where
            PromoteComputer<Self>: DelegateComponent<HandlerRefComponent>,
            <PromoteComputer<
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
    fn add_with_error(a: u64, b: u64) -> Result<u64, String> {
        a.checked_add(b).ok_or_else(|| "Overflow".to_string())
    }

    expand_add_with_error(output) {
        insta::assert_snapshot!(output, @r#"
        fn add_with_error(a: u64, b: u64) -> Result<u64, String> {
            a.checked_add(b).ok_or_else(|| "Overflow".to_string())
        }
        impl<__Context__, __Code__> Computer<__Context__, __Code__, (u64, u64)>
        for AddWithError {
            type Output = Result<u64, String>;
            fn compute(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0, arg_1): (u64, u64),
            ) -> Self::Output {
                add_with_error(arg_0, arg_1)
            }
        }
        impl<
            __Context__,
            __Code__,
        > IsProviderFor<ComputerComponent, __Context__, (__Code__, (u64, u64))>
        for AddWithError {}
        pub struct AddWithError;
        impl DelegateComponent<ComputerRefComponent> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<ComputerRefComponent>,
        {
            type Delegate = <PromoteTryComputer<
                Self,
            > as DelegateComponent<ComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<ComputerRefComponent, __Context__, __Params__> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<ComputerRefComponent>,
            <PromoteTryComputer<
                Self,
            > as DelegateComponent<
                ComputerRefComponent,
            >>::Delegate: IsProviderFor<ComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerComponent> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<TryComputerComponent>,
        {
            type Delegate = <PromoteTryComputer<
                Self,
            > as DelegateComponent<TryComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerComponent, __Context__, __Params__> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<TryComputerComponent>,
            <PromoteTryComputer<
                Self,
            > as DelegateComponent<
                TryComputerComponent,
            >>::Delegate: IsProviderFor<TryComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerRefComponent> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<TryComputerRefComponent>,
        {
            type Delegate = <PromoteTryComputer<
                Self,
            > as DelegateComponent<TryComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerRefComponent, __Context__, __Params__> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<TryComputerRefComponent>,
            <PromoteTryComputer<
                Self,
            > as DelegateComponent<
                TryComputerRefComponent,
            >>::Delegate: IsProviderFor<TryComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerComponent> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<AsyncComputerComponent>,
        {
            type Delegate = <PromoteTryComputer<
                Self,
            > as DelegateComponent<AsyncComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerComponent, __Context__, __Params__> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<AsyncComputerComponent>,
            <PromoteTryComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerComponent,
            >>::Delegate: IsProviderFor<AsyncComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerRefComponent> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
        {
            type Delegate = <PromoteTryComputer<
                Self,
            > as DelegateComponent<AsyncComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
            <PromoteTryComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerRefComponent,
            >>::Delegate: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<HandlerComponent>,
        {
            type Delegate = <PromoteTryComputer<
                Self,
            > as DelegateComponent<HandlerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<HandlerComponent>,
            <PromoteTryComputer<
                Self,
            > as DelegateComponent<
                HandlerComponent,
            >>::Delegate: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<HandlerRefComponent>,
        {
            type Delegate = <PromoteTryComputer<
                Self,
            > as DelegateComponent<HandlerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for AddWithError
        where
            PromoteTryComputer<Self>: DelegateComponent<HandlerRefComponent>,
            <PromoteTryComputer<
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

    assert_eq!(Add::compute(&app, PhantomData::<()>, (1, 2)), 3);

    assert_eq!(Add::try_compute(&app, PhantomData::<()>, (1, 2)), Ok(3));

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
        AddWithError::compute(&app, PhantomData::<()>, (1, 2)),
        Ok(3),
    );

    assert_eq!(
        AddWithError::try_compute(&app, PhantomData::<()>, (1, 2)),
        Ok(3),
    );

    assert_eq!(
        AddWithError::try_compute(&app, PhantomData::<()>, (u64::MAX, 1)),
        Err("Overflow".to_string()),
    );

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
fn to_string_ref<Value: Display>(value: &Value) -> String {
    value.to_string()
}

#[test]
fn test_computer_ref() {
    let app = App;
    let code = PhantomData::<()>;

    assert_eq!(ToStringRef::compute(&app, code, &1), "1");

    assert_eq!(ToStringRef::compute_ref(&app, code, &1), "1");

    assert_eq!(ToStringRef::try_compute(&app, code, &1), Ok("1".to_owned()));

    assert_eq!(
        ToStringRef::try_compute_ref(&app, code, &1),
        Ok("1".to_owned())
    );

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

snapshot_cgp_computer! {
    #[cgp_computer]
    pub fn add_generic<T: core::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }

    expand_add_generic(output) {
        insta::assert_snapshot!(output, @"
        pub fn add_generic<T: core::ops::Add<Output = T>>(a: T, b: T) -> T {
            a + b
        }
        impl<
            T: core::ops::Add<Output = T>,
            __Context__,
            __Code__,
        > Computer<__Context__, __Code__, (T, T)> for AddGeneric {
            type Output = T;
            fn compute(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0, arg_1): (T, T),
            ) -> Self::Output {
                add_generic(arg_0, arg_1)
            }
        }
        impl<
            T: core::ops::Add<Output = T>,
            __Context__,
            __Code__,
        > IsProviderFor<ComputerComponent, __Context__, (__Code__, (T, T))> for AddGeneric {}
        pub struct AddGeneric;
        impl DelegateComponent<ComputerRefComponent> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<ComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<ComputerRefComponent, __Context__, __Params__> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                ComputerRefComponent,
            >>::Delegate: IsProviderFor<ComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerComponent> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<TryComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerComponent, __Context__, __Params__> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerComponent,
            >>::Delegate: IsProviderFor<TryComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerRefComponent> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<TryComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerRefComponent, __Context__, __Params__> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerRefComponent,
            >>::Delegate: IsProviderFor<TryComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerComponent> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<AsyncComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerComponent, __Context__, __Params__> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerComponent,
            >>::Delegate: IsProviderFor<AsyncComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerRefComponent> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<AsyncComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerRefComponent,
            >>::Delegate: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<HandlerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerComponent,
            >>::Delegate: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<HandlerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<HandlerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for AddGeneric
        where
            PromoteComputer<Self>: DelegateComponent<HandlerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerRefComponent,
            >>::Delegate: IsProviderFor<HandlerRefComponent, __Context__, __Params__>,
        {}
        ")
    }
}
