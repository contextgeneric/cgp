#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use MyPresetA::re_exports::*;

    use crate::tests::preset::basic::components::{BarGetterComponent, FooGetterComponent};
    use crate::tests::preset::inheritance::preset_a::MyPresetA;

    MyPresetA::with_components! {
        | Components | {
            cgp_preset! {
                MyPresetB {
                    FooGetterComponent:
                        UseField<symbol!("foo")>,
                    BarGetterComponent:
                        UseField<symbol!("bar")>,
                    Components: MyPresetA::Provider,
                }
            }
        }
    }
}
