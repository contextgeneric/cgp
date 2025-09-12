#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::basic::components::{BarGetterComponent, FooGetterComponent};
    use crate::tests::preset::inheritance::preset_a::MyPresetA;

    cgp_preset! {
        MyPresetB: MyPresetA {
            FooGetterComponent:
                UseField<Symbol!("foo")>,
            BarGetterComponent:
                UseField<Symbol!("bar")>,
        }
    }

    pub trait CheckDelegatesForMyPresetB:
        DelegateComponent<FooTypeProviderComponent, Delegate = UseType<()>>
        + DelegateComponent<BarTypeProviderComponent, Delegate = UseType<()>>
        + DelegateComponent<FooGetterComponent, Delegate = UseField<Symbol!("foo")>>
        + DelegateComponent<BarGetterComponent, Delegate = UseField<Symbol!("bar")>>
    {
    }

    impl CheckDelegatesForMyPresetB for MyPresetB::Provider {}
}
