#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::basic::components::{FooGetterComponent, FooTypeProviderComponent};

    cgp_preset! {
        NestedPresetA {
            FooTypeProviderComponent: UseType<()>,
            FooGetterComponent: UseField<symbol!("foo")>,
        }
    }

    pub trait CheckDelegatesForNestedPresetA:
        DelegateComponent<FooTypeProviderComponent, Delegate = UseType<()>>
        + DelegateComponent<FooGetterComponent, Delegate = UseField<symbol!("foo")>>
    {
    }

    impl CheckDelegatesForNestedPresetA for NestedPresetA::Provider {}
}
