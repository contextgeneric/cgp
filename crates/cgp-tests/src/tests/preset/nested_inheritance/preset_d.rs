#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::nested_inheritance::preset_b::NestedPresetB;
    use crate::tests::preset::nested_inheritance::preset_c::NestedPresetC;

    cgp_preset! {
        NestedPresetD: NestedPresetB + NestedPresetC {
            override FooGetterComponent:
                UseField<symbol!("fool")>,
            override BarTypeProviderComponent:
                NestedPresetC::Provider,
        }
    }

    pub trait CheckDelegatesForNestedPresetD:
        DelegateComponent<FooTypeProviderComponent, Delegate = NestedPresetB::Provider>
        + DelegateComponent<BarTypeProviderComponent, Delegate = NestedPresetC::Provider>
        + DelegateComponent<BarGetterComponent, Delegate = NestedPresetC::Provider>
        + DelegateComponent<FooGetterComponent, Delegate = UseField<symbol!("fool")>>
    {
    }

    impl CheckDelegatesForNestedPresetD for NestedPresetD::Provider {}
}
