use cgp::prelude::*;

use crate::tests::preset::basic::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::tests::preset::basic::preset::{CheckDelegatesForMyPreset, MyPreset};

#[cgp_context(MyContextComponents: MyPreset)]
#[derive(HasField)]
pub struct MyContext {
    pub foo: (),
    pub bar: (),
}

delegate_components! {
    MyContextComponents {
        BarGetterComponent: UseField<symbol!("bar")>,
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
