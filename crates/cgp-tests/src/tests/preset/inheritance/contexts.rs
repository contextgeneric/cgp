use cgp::prelude::*;

use crate::tests::preset::basic::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::tests::preset::inheritance::preset_b::{CheckDelegatesForMyPresetB, MyPresetB};

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
