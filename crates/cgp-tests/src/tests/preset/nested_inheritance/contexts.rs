use cgp::prelude::*;

use crate::tests::preset::basic::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::tests::preset::nested_inheritance::preset_d::{
    CheckDelegatesForNestedPresetD, NestedPresetD,
};

#[cgp_context(MyContextComponents: NestedPresetD)]
#[derive(HasField)]
pub struct MyContext {
    pub fool: (),
    pub bar: (),
}

check_components! {
    CanUseMyContext for MyContext {
        FooTypeProviderComponent,
        BarTypeProviderComponent,
        FooGetterComponent,
        BarGetterComponent,
    }
}

impl CheckDelegatesForNestedPresetD for MyContextComponents {}
