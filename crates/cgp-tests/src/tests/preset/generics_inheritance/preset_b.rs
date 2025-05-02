#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use MyGenericPresetA::re_exports::*;

    use crate::tests::preset::generics_inheritance::preset_a::MyGenericPresetA;

    MyGenericPresetA::with_components! {
        [
            FooGetterComponent,
        ],
        | Components | {
            cgp_preset! {
                MyGenericPresetB<T> {
                    Components: MyGenericPresetA::Provider<T>,
                    <I> FooGetterComponent<I>:
                        UseField<symbol!("foo")>,
                }
            }
        }
    }
}
