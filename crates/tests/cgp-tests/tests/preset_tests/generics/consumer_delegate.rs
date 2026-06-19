use cgp::prelude::*;
use cgp_macro_test_util::snapshot_check_components;

use crate::preset_tests::generics::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::preset_tests::generics::preset::MyGenericPreset;

#[cgp_inherit(MyGenericPreset<T>)]
#[derive(HasField)]
pub struct MyContext<T> {
    pub foo: T,
    pub bar: T,
}

snapshot_check_components! {
    check_components! {
        #[check_trait(CanUseMyContext)]
        <T> MyContext<T> {
            FooTypeProviderComponent,
            BarTypeProviderComponent,
            BarGetterComponent,
        }
    }

    expand_check_my_context(output) {
        insta::assert_snapshot!(output, @"
        trait CanUseMyContext<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl<T> CanUseMyContext<FooTypeProviderComponent, ()> for MyContext<T> {}
        impl<T> CanUseMyContext<BarTypeProviderComponent, ()> for MyContext<T> {}
        impl<T> CanUseMyContext<BarGetterComponent, ()> for MyContext<T> {}
        ")
    }
}

snapshot_check_components! {
    check_components! {
        #[check_trait(CanUseFooGetter)]
        <const I: usize, T> MyContext<T> {
            FooGetterComponent<Index<I>>: Index<I>,
        }
    }

    expand_check_my_context_2(output) {
        insta::assert_snapshot!(output, @"
        trait CanUseFooGetter<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl<const I: usize, T> CanUseFooGetter<FooGetterComponent<Index<I>>, Index<I>>
        for MyContext<T> {}
        ")
    }
}
