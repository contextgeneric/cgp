#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::preset_tests::basic::components::{FooGetterComponent, FooTypeProviderComponent};

    cgp_preset! {
        NestedPresetA {
            FooTypeProviderComponent: UseType<()>,
            FooGetterComponent: UseField<Symbol!("foo")>,
        }
    }

    pub trait CheckDelegatesForNestedPresetA:
        DelegateComponent<FooTypeProviderComponent, Delegate = UseType<()>>
        + DelegateComponent<FooGetterComponent, Delegate = UseField<Symbol!("foo")>>
    {
    }

    impl CheckDelegatesForNestedPresetA for NestedPresetA::Provider {}
}
