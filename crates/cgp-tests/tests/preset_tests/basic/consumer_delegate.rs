use cgp::prelude::*;

use crate::preset_tests::basic::components::{
    BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent, HasBar,
};
use crate::preset_tests::basic::preset::{CheckDelegatesForMyPreset, MyPreset};

#[derive(HasField)]
pub struct MyContext {
    pub foo: (),
    pub bar: (),
}

impl<__Name__> DelegateComponent<__Name__> for MyContext
where
    Self: MyPreset::IsPreset<__Name__>,
    MyPreset::Components: DelegateComponent<__Name__>,
{
    type Delegate = <MyPreset::Components as DelegateComponent<__Name__>>::Delegate;
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
