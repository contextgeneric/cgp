#[cgp::re_export_imports]
mod preset {
    #![allow(unused_imports)]

    use cgp::prelude::*;
    use MyGenericPresetA::components::*;

    use crate::tests::preset::generics_inheritance::preset_a::MyGenericPresetA;

    MyGenericPresetA::with_components! {
        [
            FooGetterComponent,
        ],
        | Components | {
            cgp_preset! {
                MyGenericPresetB<T> {
                    Components: MyGenericPresetA::Provider<T>,
                    override <I> FooGetterComponent<I>:
                        UseField<symbol!("foo")>,
                }
            }
        }
    }
}
