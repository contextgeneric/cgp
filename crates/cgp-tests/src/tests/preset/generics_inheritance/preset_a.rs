#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::generics_inheritance::components::{
        BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
    };

    cgp_preset! {
        MyGenericPresetA<T> {
            [
                FooTypeProviderComponent,
                BarTypeProviderComponent,
            ]:
                UseType<T>,
            <const I: usize> FooGetterComponent<Index<I>>:
                UseField<Symbol!("foo")>,
            <I> BarGetterComponent<I>:
                UseField<Symbol!("bar")>,
        }
    }
}
