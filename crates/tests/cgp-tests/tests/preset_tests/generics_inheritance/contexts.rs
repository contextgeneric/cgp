use cgp::prelude::*;

use crate::preset_tests::generics_inheritance::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::preset_tests::generics_inheritance::preset_b::MyGenericPresetB;

#[cgp_inherit(MyGenericPresetB<()>)]
#[derive(HasField)]
pub struct MyContext {
    pub food: (),
    pub bar: (),
}

check_components! {
    MyContext {
        FooTypeProviderComponent,
        BarTypeProviderComponent,
    }
}

check_components! {
    #[check_trait(CanUseFooGetter)]
    <I> MyContext {
        [
            FooGetterComponent<I>,
            BarGetterComponent<I>,
        ]: I,
    }
}
