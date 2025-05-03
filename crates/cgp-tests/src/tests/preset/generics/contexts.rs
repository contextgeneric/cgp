use cgp::prelude::*;

use crate::tests::preset::generics::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::tests::preset::generics::preset::MyGenericPreset;

#[cgp_context(MyContextComponents<T>: MyGenericPreset<T>)]
#[derive(HasField)]
pub struct MyContext<T> {
    pub foo: T,
    pub bar: T,
}

check_components! {
    <T> CanUseMyContext for MyContext<T> {
        FooTypeProviderComponent,
        BarTypeProviderComponent,
        BarGetterComponent,
    }
}

check_components! {
    <const I: usize, T>
    CanUseFooGetter for MyContext<T> {
        FooGetterComponent<Index<I>>: Index<I>,
    }
}
