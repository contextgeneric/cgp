use cgp::prelude::*;

use crate::tests::preset::generics::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::tests::preset::generics::preset::MyGenericPreset;

#[cgp_context(MyContextComponents: MyGenericPreset<()>)]
#[derive(HasField)]
pub struct MyContext {
    pub foo: (),
    pub bar: (),
}

check_components! {
    CanUseMyContext for MyContext {
        FooTypeProviderComponent,
        BarTypeProviderComponent,
        BarGetterComponent,
    }
}

check_components! {
    <const I: usize>
    CanUseFooGetter for MyContext {
        FooGetterComponent<Index<I>>: Index<I>,
    }
}
