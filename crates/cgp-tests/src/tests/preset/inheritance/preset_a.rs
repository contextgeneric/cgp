#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::basic::components::{
        BarTypeProviderComponent, FooTypeProviderComponent,
    };

    cgp_preset! {
        MyPresetA {
            [
                FooTypeProviderComponent,
                BarTypeProviderComponent,
            ]:
                UseType<()>,
        }
    }

    pub trait CheckDelegatesForMyPresetA:
        DelegateComponent<FooTypeProviderComponent, Delegate = UseType<()>>
        + DelegateComponent<BarTypeProviderComponent, Delegate = UseType<()>>
    {
    }

    impl CheckDelegatesForMyPresetA for MyPresetA::Provider {}
}
