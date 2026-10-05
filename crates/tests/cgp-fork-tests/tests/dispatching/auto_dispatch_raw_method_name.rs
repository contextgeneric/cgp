//! A dispatch trait whose method has a raw identifier as its name, `r#type`. The
//! generated computer and helper are named from the unrawed name (`ComputeType` and
//! `__compute_type__`), since `r#` cannot appear inside a longer identifier.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

use cgp_fork::prelude::*;
use cgp_fork_macro_test_util::snapshot_cgp_auto_dispatch;

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

pub struct Circle;

pub struct Square;

snapshot_cgp_auto_dispatch! {
    #[cgp_auto_dispatch]
    pub trait HasKind {
        fn r#type(&self) -> &'static str;
    }

    expand_raw_method_name(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasKind {
            fn r#type(&self) -> &'static str;
        }
        impl<__Variants__> HasKind for __Variants__
        where
            MatchWithValueHandlersRef<
                ComputeType,
            >: for<'__a__> Computer<(), (), &'__a__ __Variants__, Output = &'static str>,
            __Variants__: HasExtractor,
        {
            fn r#type(&self) -> &'static str {
                <MatchWithValueHandlersRef<
                    ComputeType,
                > as Computer<_, _, _>>::compute(&(), ::core::marker::PhantomData::<()>, self)
            }
        }
        fn __compute_type__<'__a__, __Variants__: HasKind>(
            __Variants__: &'__a__ __Variants__,
        ) -> &'static str {
            __Variants__.r#type()
        }
        impl<
            '__a__,
            __Variants__: HasKind,
            __Context__,
            __Code__,
        > Computer<__Context__, __Code__, (&'__a__ __Variants__)> for ComputeType {
            type Output = &'static str;
            fn compute(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0): (&'__a__ __Variants__),
            ) -> Self::Output {
                __compute_type__(arg_0)
            }
        }
        impl<
            '__a__,
            __Variants__: HasKind,
            __Context__,
            __Code__,
        > IsProviderFor<ComputerComponent, __Context__, (__Code__, (&'__a__ __Variants__))>
        for ComputeType {}
        pub struct ComputeType;
        impl DelegateComponent<ComputerRefComponent> for ComputeType
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
        > IsProviderFor<ComputerRefComponent, __Context__, __Params__> for ComputeType
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                ComputerRefComponent,
            >>::Delegate: IsProviderFor<ComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerComponent> for ComputeType
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
        > IsProviderFor<TryComputerComponent, __Context__, __Params__> for ComputeType
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerComponent,
            >>::Delegate: IsProviderFor<TryComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerRefComponent> for ComputeType
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
        > IsProviderFor<TryComputerRefComponent, __Context__, __Params__> for ComputeType
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerRefComponent,
            >>::Delegate: IsProviderFor<TryComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerComponent> for ComputeType
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
        > IsProviderFor<AsyncComputerComponent, __Context__, __Params__> for ComputeType
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerComponent,
            >>::Delegate: IsProviderFor<AsyncComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerRefComponent> for ComputeType
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
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for ComputeType
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerRefComponent,
            >>::Delegate: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for ComputeType
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
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for ComputeType
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerComponent,
            >>::Delegate: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for ComputeType
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
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for ComputeType
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

impl HasKind for Circle {
    fn r#type(&self) -> &'static str {
        "circle"
    }
}

impl HasKind for Square {
    fn r#type(&self) -> &'static str {
        "square"
    }
}

#[test]
fn test_raw_method_name_dispatches() {
    assert_eq!(Shape::Circle(Circle).r#type(), "circle");
    assert_eq!(Shape::Square(Square).r#type(), "square");
}
