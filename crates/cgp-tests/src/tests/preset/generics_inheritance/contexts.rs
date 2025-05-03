use cgp::prelude::*;

use crate::tests::preset::generics_inheritance::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::tests::preset::generics_inheritance::preset_b::MyGenericPresetB;

#[cgp_context(MyContextComponents: MyGenericPresetB<()>)]
#[derive(HasField)]
pub struct MyContext {
    pub food: (),
    pub bar: (),
}

check_components! {
    CanUseMyContext for MyContext {
        FooTypeProviderComponent,
        BarTypeProviderComponent,
    }
}

check_components! {
    <I>
    CanUseFooGetter for MyContext {
        [
            FooGetterComponent<I>,
            BarGetterComponent<I>,
        ]: I,
    }
}
