#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::basic::components::{
        BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
    };

    cgp_preset! {
        MyPreset {
            [
                FooTypeProviderComponent,
                BarTypeProviderComponent,
            ]:
                UseType<()>,
            FooGetterComponent:
                UseField<Symbol!("foo")>,
        }
    }

    pub trait CheckDelegatesForMyPreset:
        DelegateComponent<FooTypeProviderComponent, Delegate = UseType<()>>
        + DelegateComponent<BarTypeProviderComponent, Delegate = UseType<()>>
        + DelegateComponent<FooGetterComponent, Delegate = UseField<Symbol!("foo")>>
    {
    }

    impl CheckDelegatesForMyPreset for MyPreset::Provider {}
}
