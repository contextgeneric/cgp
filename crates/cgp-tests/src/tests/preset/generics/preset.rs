#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::generics::components::{
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
                UseField<symbol!("foo")>,
            BarGetterComponent:
                UseField<symbol!("bar")>,
        }
    }
}
