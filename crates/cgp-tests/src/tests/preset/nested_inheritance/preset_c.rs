#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::basic::components::{BarGetterComponent, BarTypeProviderComponent};

    cgp_preset! {
        NestedPresetC {
            BarTypeProviderComponent: UseType<()>,
            BarGetterComponent: UseField<symbol!("bar")>,
        }
    }
}
