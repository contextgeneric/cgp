use cgp::prelude::*;

use crate::tests::preset::basic::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::tests::preset::inheritance::preset_b::MyPresetB;

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

pub trait CheckDelegatesForMyContextComponents:
    DelegateComponent<FooTypeProviderComponent, Delegate = MyPresetB::Provider>
    + DelegateComponent<BarTypeProviderComponent, Delegate = MyPresetB::Provider>
    + DelegateComponent<FooGetterComponent, Delegate = MyPresetB::Provider>
    + DelegateComponent<BarGetterComponent, Delegate = MyPresetB::Provider>
{
}

impl CheckDelegatesForMyContextComponents for MyContextComponents {}
