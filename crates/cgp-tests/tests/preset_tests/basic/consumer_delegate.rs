use cgp::prelude::*;

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

check_components! {
    CanUseMyContext for MyContext {
        FooTypeProviderComponent,
        BarTypeProviderComponent,
        FooGetterComponent,
    }
}

impl CheckDelegatesForMyPreset for MyContext {}
