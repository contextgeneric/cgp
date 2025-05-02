#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::basic::components::{BarGetterComponent, FooGetterComponent};
    use crate::tests::preset::inheritance::preset_a::MyPresetA;

    cgp_preset! {
        MyPresetB: MyPresetA {
            FooGetterComponent:
                UseField<symbol!("foo")>,
            BarGetterComponent:
                UseField<symbol!("bar")>,
        }
    }
}
