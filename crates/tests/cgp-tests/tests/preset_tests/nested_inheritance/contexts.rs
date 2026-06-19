use cgp::prelude::*;
use cgp_macro_test_util::snapshot_check_components;

use crate::preset_tests::basic::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::preset_tests::nested_inheritance::preset_d::{
    CheckDelegatesForNestedPresetD, NestedPresetD,
};

#[cgp_inherit(NestedPresetD)]
#[derive(HasField)]
pub struct MyContext {
    pub fool: (),
    pub bar: (),
}

snapshot_check_components! {
    check_components! {
        MyContext {
            FooTypeProviderComponent,
            BarTypeProviderComponent,
            FooGetterComponent,
            BarGetterComponent,
        }
    }

    expand_check_my_context(output) {
        insta::assert_snapshot!(output, @"
        trait __CheckMyContext<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CheckMyContext<FooTypeProviderComponent, ()> for MyContext {}
        impl __CheckMyContext<BarTypeProviderComponent, ()> for MyContext {}
        impl __CheckMyContext<FooGetterComponent, ()> for MyContext {}
        impl __CheckMyContext<BarGetterComponent, ()> for MyContext {}
        ")
    }
}

impl CheckDelegatesForNestedPresetD for MyContext {}
