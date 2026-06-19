use cgp::prelude::*;
use cgp_macro_test_util::snapshot_check_components;

use crate::preset_tests::basic::components::{
    BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent, HasBar,
};
use crate::preset_tests::basic::preset::{CheckDelegatesForMyPreset, MyPreset};

#[cgp_inherit(MyPreset)]
#[derive(HasField)]
pub struct MyContext {
    pub foo: (),
    pub bar: (),
}

impl HasBar for MyContext {
    fn bar(&self) -> &Self::Bar {
        &self.bar
    }
}

snapshot_check_components! {
    check_components! {
        MyContext {
            FooTypeProviderComponent,
            BarTypeProviderComponent,
            FooGetterComponent,
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
        ")
    }
}

impl CheckDelegatesForMyPreset for MyContext {}
