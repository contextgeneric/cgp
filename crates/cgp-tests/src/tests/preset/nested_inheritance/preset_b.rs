#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::basic::components::FooGetterComponent;
    use crate::tests::preset::nested_inheritance::preset_a::NestedPresetA;

    cgp_preset! {
        NestedPresetB: NestedPresetA {
            override FooGetterComponent:
                UseField<symbol!("food")>,
        }
    }

    pub trait CheckDelegatesForNestedPresetB:
        DelegateComponent<FooTypeProviderComponent, Delegate = NestedPresetA::Provider>
        + DelegateComponent<FooGetterComponent, Delegate = UseField<symbol!("food")>>
    {
    }

    impl CheckDelegatesForNestedPresetB for NestedPresetB::Provider {}
}
