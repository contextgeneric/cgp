#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::basic::components::FooTypeProviderComponent;

    cgp_preset! {
        NestedPresetA {
            FooTypeProviderComponent: UseType<()>,
        }
    }
}
