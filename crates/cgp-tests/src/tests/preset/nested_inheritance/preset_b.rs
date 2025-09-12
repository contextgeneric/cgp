#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::basic::components::{BarTypeProviderComponent, FooGetterComponent};
    use crate::tests::preset::nested_inheritance::preset_a::NestedPresetA;

    cgp_preset! {
        NestedPresetB: NestedPresetA {
            override FooGetterComponent:
                UseField<Symbol!("food")>,
            BarTypeProviderComponent: UseType<()>,
        }
    }

    pub trait CheckDelegatesForNestedPresetB:
        DelegateComponent<FooTypeProviderComponent, Delegate = UseType<()>>
        + DelegateComponent<FooGetterComponent, Delegate = UseField<Symbol!("food")>>
    {
    }

    impl CheckDelegatesForNestedPresetB for NestedPresetB::Provider {}
}
