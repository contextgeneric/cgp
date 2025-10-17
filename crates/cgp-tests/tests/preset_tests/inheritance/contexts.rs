use cgp::prelude::*;

use crate::preset_tests::basic::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::preset_tests::inheritance::preset_b::{CheckDelegatesForMyPresetB, MyPresetB};

#[cgp_context(MyContextComponents: MyPresetB)]
#[derive(HasField)]
pub struct MyContext {
    pub foo: (),
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

impl CheckDelegatesForMyPresetB for MyContextComponents {}
