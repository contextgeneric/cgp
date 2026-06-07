#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::preset_tests::generics::components::{
        BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
    };

    cgp_preset! {
        MyGenericPreset<T> {
            [
                FooTypeProviderComponent,
                BarTypeProviderComponent,
            ]:
                UseType<T>,
            <const I: usize> FooGetterComponent<Index<I>>:
                UseField<Symbol!("foo")>,
            BarGetterComponent:
                UseField<Symbol!("bar")>,
        }
    }
}
