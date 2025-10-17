use cgp::prelude::*;

use crate::preset_tests::basic::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::preset_tests::basic::preset::{CheckDelegatesForMyPreset, MyPreset};

#[cgp_context(MyContextComponents: MyPreset)]
#[derive(HasField)]
pub struct MyContext {
    pub foo: (),
    pub bar: (),
}

delegate_components! {
    MyContextComponents {
        BarGetterComponent: UseField<Symbol!("bar")>,
    }
}

check_components! {
    CanUseMyContext for MyContext {
        FooTypeProviderComponent,
        BarTypeProviderComponent,
        FooGetterComponent,
        BarGetterComponent,
    }
}

impl CheckDelegatesForMyPreset for MyContextComponents {}
