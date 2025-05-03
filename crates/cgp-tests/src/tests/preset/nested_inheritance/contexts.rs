use cgp::prelude::*;

use crate::tests::preset::basic::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::tests::preset::nested_inheritance::preset_d::NestedPresetD;

#[cgp_context(MyContextComponents: NestedPresetD)]
#[derive(HasField)]
pub struct MyContext {
    pub fool: (),
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
    DelegateComponent<FooTypeProviderComponent, Delegate = NestedPresetD::Provider>
    + DelegateComponent<BarTypeProviderComponent, Delegate = NestedPresetD::Provider>
    + DelegateComponent<FooGetterComponent, Delegate = NestedPresetD::Provider>
    + DelegateComponent<BarGetterComponent, Delegate = NestedPresetD::Provider>
{
}

impl CheckDelegatesForMyContextComponents for MyContextComponents {}
