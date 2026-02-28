use cgp::prelude::*;

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

check_components! {
    MyContext {
        FooTypeProviderComponent,
        BarTypeProviderComponent,
        FooGetterComponent,
        BarGetterComponent,
    }
}

impl CheckDelegatesForNestedPresetD for MyContext {}
