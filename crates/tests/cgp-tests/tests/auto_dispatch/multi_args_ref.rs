//! `#[cgp_auto_dispatch]` on a `&mut self` method with borrowed arguments and a
//! borrowed return, exercising the lifetime handling in the generated handler.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;
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
        #[cgp_computer(ComputeCall)]
        fn __compute_call__<'a, __Variants__: CanCall>(
            __Variants__: &'a mut __Variants__,
            (arg_0, arg_1): (&'a u64, bool),
        ) -> &'a str {
            __Variants__.call(arg_0, arg_1)
        }
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
