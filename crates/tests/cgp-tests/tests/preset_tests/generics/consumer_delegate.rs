use cgp::prelude::*;

use crate::preset_tests::generics::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::preset_tests::generics::preset::MyGenericPreset;

#[cgp_inherit(MyGenericPreset<T>)]
#[derive(HasField)]
pub struct MyContext<T> {
    pub foo: T,
    pub bar: T,
}

check_components! {
    #[check_trait(CanUseMyContext)]
    <T> MyContext<T> {
        FooTypeProviderComponent,
        BarTypeProviderComponent,
        BarGetterComponent,
    }
}

check_components! {
    #[check_trait(CanUseFooGetter)]
    <const I: usize, T> MyContext<T> {
        FooGetterComponent<Index<I>>: Index<I>,
    }
}
