#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::preset_tests::basic::components::{BarGetterComponent, BarTypeProviderComponent};

    cgp_preset! {
        NestedPresetC {
            BarTypeProviderComponent: UseType<()>,
            BarGetterComponent: UseField<Symbol!("bar")>,
        }
    }

    pub trait CheckDelegatesForNestedPresetC:
        DelegateComponent<BarTypeProviderComponent, Delegate = UseType<()>>
        + DelegateComponent<BarGetterComponent, Delegate = UseField<Symbol!("bar")>>
    {
    }

    impl CheckDelegatesForNestedPresetC for NestedPresetC::Provider {}
}
