#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::preset_tests::nested_inheritance::preset_b::NestedPresetB;
    use crate::preset_tests::nested_inheritance::preset_c::NestedPresetC;

    cgp_preset! {
        NestedPresetD: NestedPresetB + NestedPresetC {
            override FooGetterComponent:
                UseField<Symbol!("fool")>,
            override BarTypeProviderComponent ->
                NestedPresetC::Provider,
        }
    }

    pub trait CheckDelegatesForNestedPresetD:
        DelegateComponent<FooTypeProviderComponent, Delegate = UseType<()>>
        + DelegateComponent<BarTypeProviderComponent, Delegate = UseType<()>>
        + DelegateComponent<BarGetterComponent, Delegate = UseField<Symbol!("bar")>>
        + DelegateComponent<FooGetterComponent, Delegate = UseField<Symbol!("fool")>>
    {
    }

    impl CheckDelegatesForNestedPresetD for NestedPresetD::Provider {}
}
