//! A dispatch trait with a supertrait.
//!
//! The blanket impl requires the trait's supertraits of the enum, so the enum
//! must provide them. Here the supertrait is itself a dispatch trait, so the enum
//! gets it through that trait's own blanket impl, and the subtrait's per-payload
//! impls can call the supertrait's method.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

use cgp_fork::prelude::*;
use cgp_fork_macro_test_util::snapshot_cgp_auto_dispatch;

pub struct Circle;

pub struct Square;

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

#[cgp_auto_dispatch]
pub trait HasName {
    fn name(&self) -> &'static str;
}

snapshot_cgp_auto_dispatch! {
    #[cgp_auto_dispatch]
    pub trait CanGreet: HasName {
        fn greet(&self) -> String;
    }

    expand_supertrait(output) {
        insta::assert_snapshot!(output, @"
        pub trait CanGreet: HasName {
            fn greet(&self) -> String;
        }
        impl<__Variants__> CanGreet for __Variants__
        where
            MatchWithValueHandlersRef<
                ComputeGreet,
            >: for<'__a__> Computer<(), (), &'__a__ __Variants__, Output = String>,
            __Variants__: HasExtractor,
            __Variants__: HasName,
        {
            fn greet(&self) -> String {
                <MatchWithValueHandlersRef<
                    ComputeGreet,
                > as Computer<_, _, _>>::compute(&(), ::core::marker::PhantomData::<()>, self)
            }
        }
        fn __compute_greet__<'__a__, __Variants__: CanGreet>(
            __Variants__: &'__a__ __Variants__,
        ) -> String {
            __Variants__.greet()
        }
        impl<
            '__a__,
            __Variants__: CanGreet,
            __Context__,
            __Code__,
        > Computer<__Context__, __Code__, (&'__a__ __Variants__)> for ComputeGreet {
            type Output = String;
            fn compute(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0): (&'__a__ __Variants__),
            ) -> Self::Output {
                __compute_greet__(arg_0)
            }
        }
        impl<
            '__a__,
            __Variants__: CanGreet,
            __Context__,
            __Code__,
        > IsProviderFor<ComputerComponent, __Context__, (__Code__, (&'__a__ __Variants__))>
        for ComputeGreet {}
        pub struct ComputeGreet;
        impl DelegateComponent<ComputerRefComponent> for ComputeGreet
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
        > IsProviderFor<ComputerRefComponent, __Context__, __Params__> for ComputeGreet
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                ComputerRefComponent,
            >>::Delegate: IsProviderFor<ComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerComponent> for ComputeGreet
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
        > IsProviderFor<TryComputerComponent, __Context__, __Params__> for ComputeGreet
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerComponent,
            >>::Delegate: IsProviderFor<TryComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerRefComponent> for ComputeGreet
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
        > IsProviderFor<TryComputerRefComponent, __Context__, __Params__> for ComputeGreet
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerRefComponent,
            >>::Delegate: IsProviderFor<TryComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerComponent> for ComputeGreet
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
        > IsProviderFor<AsyncComputerComponent, __Context__, __Params__> for ComputeGreet
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerComponent,
            >>::Delegate: IsProviderFor<AsyncComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerRefComponent> for ComputeGreet
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
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for ComputeGreet
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerRefComponent,
            >>::Delegate: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for ComputeGreet
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
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for ComputeGreet
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerComponent,
            >>::Delegate: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for ComputeGreet
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
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for ComputeGreet
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

impl HasName for Circle {
    fn name(&self) -> &'static str {
        "circle"
    }
}

impl HasName for Square {
    fn name(&self) -> &'static str {
        "square"
    }
}

impl CanGreet for Circle {
    fn greet(&self) -> String {
        format!("hello, {}", self.name())
    }
}

impl CanGreet for Square {
    fn greet(&self) -> String {
        format!("hi, {}", self.name())
    }
}

#[test]
fn test_supertrait() {
    assert_eq!(Shape::Square(Square).greet(), "hi, square");
}
