#[cgp::re_export_imports]
mod preset {
    #![allow(unused_imports)]

    use cgp::prelude::*;

    use crate::tests::preset::generics_inheritance::preset_a::MyGenericPresetA;

    cgp_preset! {
        MyGenericPresetB<T>: MyGenericPresetA<T> {
            override <I> FooGetterComponent<I>:
                UseField<Symbol!("food")>,
        }
    }
}
