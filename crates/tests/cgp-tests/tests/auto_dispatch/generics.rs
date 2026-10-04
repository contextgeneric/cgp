//! `#[cgp_auto_dispatch]` on a generic trait (`CanCall<T>`), where the per-variant
//! impls may add their own bounds on `T` (here `Foo` requires `T: Display`).
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_auto_dispatch.md.

use core::fmt::Display;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_dispatch;

use super::types::{Bar, Foo, FooBar};

snapshot_cgp_auto_dispatch! {
    #[cgp_auto_dispatch]
    pub trait CanCall<T> {
        fn call_a(&self, _a: u64, _b: &T) -> String;

        fn call_b(self, _a: u64, b: &mut T) -> &str;
    }

    expand_generic_trait(output) {
        insta::assert_snapshot!(output, @"
        pub trait CanCall<T> {
            fn call_a(&self, _a: u64, _b: &T) -> String;
            fn call_b(self, _a: u64, b: &mut T) -> &str;
        }
        impl<__Variants__, T> CanCall<T> for __Variants__
        where
            MatchFirstWithValueHandlersRef<
                ComputeCallA,
            >: for<'__a__> Computer<
                (),
                (),
                (&'__a__ __Variants__, (u64, &'__a__ T)),
                Output = String,
            >,
            MatchFirstWithValueHandlers<
                ComputeCallB,
            >: for<'__a__> Computer<
                (),
                (),
                (__Variants__, (u64, &'__a__ mut T)),
                Output = &'__a__ str,
            >,
            __Variants__: HasExtractor,
        {
            fn call_a(&self, arg_0: u64, arg_1: &T) -> String {
                <MatchFirstWithValueHandlersRef<
                    ComputeCallA,
                > as Computer<
                    _,
                    _,
                    _,
                >>::compute(&(), ::core::marker::PhantomData::<()>, (self, (arg_0, arg_1)))
            }
            fn call_b(self, arg_0: u64, arg_1: &mut T) -> &str {
                <MatchFirstWithValueHandlers<
                    ComputeCallB,
                > as Computer<
                    _,
                    _,
                    _,
                >>::compute(&(), ::core::marker::PhantomData::<()>, (self, (arg_0, arg_1)))
            }
        }
        #[cgp_computer(ComputeCallA)]
        fn __compute_call_a__<'__a__, __Variants__: CanCall<T>, T>(
            __Variants__: &'__a__ __Variants__,
            (arg_0, arg_1): (u64, &'__a__ T),
        ) -> String {
            __Variants__.call_a(arg_0, arg_1)
        }
        #[cgp_computer(ComputeCallB)]
        fn __compute_call_b__<'__a__, __Variants__: CanCall<T>, T>(
            __Variants__: __Variants__,
            (arg_0, arg_1): (u64, &'__a__ mut T),
        ) -> &'__a__ str {
            __Variants__.call_b(arg_0, arg_1)
        }
        ")
    }
}

impl<T: Display> CanCall<T> for Foo {
    fn call_a(&self, _a: u64, b: &T) -> String {
        format!("foo-{}", b)
    }

    fn call_b(self, _a: u64, _b: &mut T) -> &str {
        "foo"
    }
}

impl<T> CanCall<T> for Bar {
    fn call_a(&self, _a: u64, _b: &T) -> String {
        "bar".to_owned()
    }

    fn call_b(self, _a: u64, _b: &mut T) -> &str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall<String> {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_generics() {
    assert_eq!(FooBar::Foo(Foo).call_a(42, &"extra"), "foo-extra");
}
