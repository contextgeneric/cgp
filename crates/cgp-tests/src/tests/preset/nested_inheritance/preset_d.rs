#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::nested_inheritance::preset_b::NestedPresetB;
    use crate::tests::preset::nested_inheritance::preset_c::NestedPresetC;

    cgp_preset! {
        NestedPresetD: NestedPresetB + NestedPresetC {
        }
    }
}
