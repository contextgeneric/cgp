//! `#[cgp_auto_dispatch]` on a `&mut self` method with borrowed arguments and a
//! borrowed return, exercising the lifetime handling in the generated handler.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_auto_dispatch.md.

use cgp_macro_test_util::snapshot_cgp_auto_dispatch;

use super::types::{Bar, Foo, FooBar};

snapshot_cgp_auto_dispatch! {
    #[cgp_auto_dispatch]
    pub trait CanCall {
        fn call<'a>(&'a mut self, _a: &'a u64, _b: bool) -> &'a str;
    }

    expand_multi_args_ref(output) {
        insta::assert_snapshot!(output, @"
        pub trait CanCall {
            fn call<'a>(&'a mut self, _a: &'a u64, _b: bool) -> &'a str;
        }
        impl<__Variants__> CanCall for __Variants__
        where
            MatchFirstWithValueHandlersMut<
                ComputeCall,
            >: for<'a> Computer<
                (),
                (),
                (&'a mut __Variants__, (&'a u64, bool)),
                Output = &'a str,
            >,
            __Variants__: HasExtractor,
        {
            fn call<'a>(&'a mut self, arg_0: &'a u64, arg_1: bool) -> &'a str {
                <MatchFirstWithValueHandlersMut<
                    ComputeCall,
                > as Computer<
                    _,
                    _,
                    _,
                >>::compute(&(), ::core::marker::PhantomData::<()>, (self, (arg_0, arg_1)))
            }
        }
        fn __compute_call__<'a, __Variants__: CanCall>(
            __Variants__: &'a mut __Variants__,
            (arg_0, arg_1): (&'a u64, bool),
        ) -> &'a str {
            __Variants__.call(arg_0, arg_1)
        }
        impl<
            'a,
            __Variants__: CanCall,
            __Context__,
            __Code__,
        > Computer<__Context__, __Code__, (&'a mut __Variants__, (&'a u64, bool))>
        for ComputeCall {
            type Output = &'a str;
            fn compute(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0, arg_1): (&'a mut __Variants__, (&'a u64, bool)),
            ) -> Self::Output {
                __compute_call__(arg_0, arg_1)
            }
        }
        impl<
            'a,
            __Variants__: CanCall,
            __Context__,
            __Code__,
        > IsProviderFor<
            ComputerComponent,
            __Context__,
            (__Code__, (&'a mut __Variants__, (&'a u64, bool))),
        > for ComputeCall {}
        pub struct ComputeCall;
        impl DelegateComponent<ComputerRefComponent> for ComputeCall
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
        > IsProviderFor<ComputerRefComponent, __Context__, __Params__> for ComputeCall
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                ComputerRefComponent,
            >>::Delegate: IsProviderFor<ComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerComponent> for ComputeCall
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
        > IsProviderFor<TryComputerComponent, __Context__, __Params__> for ComputeCall
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerComponent,
            >>::Delegate: IsProviderFor<TryComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerRefComponent> for ComputeCall
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
        > IsProviderFor<TryComputerRefComponent, __Context__, __Params__> for ComputeCall
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerRefComponent,
            >>::Delegate: IsProviderFor<TryComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerComponent> for ComputeCall
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
        > IsProviderFor<AsyncComputerComponent, __Context__, __Params__> for ComputeCall
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerComponent,
            >>::Delegate: IsProviderFor<AsyncComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerRefComponent> for ComputeCall
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
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for ComputeCall
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerRefComponent,
            >>::Delegate: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for ComputeCall
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
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for ComputeCall
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerComponent,
            >>::Delegate: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for ComputeCall
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
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for ComputeCall
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

impl CanCall for Foo {
    fn call(&mut self, _a: &u64, _b: bool) -> &str {
        "foo"
    }
}

impl CanCall for Bar {
    fn call(&mut self, _a: &u64, _b: bool) -> &str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_multi_args_ref() {
    assert_eq!(FooBar::Foo(Foo).call(&42, true), "foo");
}
