use cgp::prelude::*;
use cgp_macro_test_util::snapshot_check_components;

use crate::preset_tests::generics_inheritance::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::preset_tests::generics_inheritance::preset_b::MyGenericPresetB;

#[cgp_inherit(MyGenericPresetB<()>)]
#[derive(HasField)]
pub struct MyContext {
    pub food: (),
    pub bar: (),
}

snapshot_check_components! {
    check_components! {
        MyContext {
            FooTypeProviderComponent,
            BarTypeProviderComponent,
        }
    }

    expand_check_my_context(output) {
        insta::assert_snapshot!(output, @"
        trait __CheckMyContext<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CheckMyContext<FooTypeProviderComponent, ()> for MyContext {}
        impl __CheckMyContext<BarTypeProviderComponent, ()> for MyContext {}
        ")
    }
}

snapshot_check_components! {
    check_components! {
        #[check_trait(CanUseFooGetter)]
        <I> MyContext {
            [
                FooGetterComponent<I>,
                BarGetterComponent<I>,
            ]: I,
        }
    }

    expand_check_my_context_2(output) {
        insta::assert_snapshot!(output, @"
        trait CanUseFooGetter<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl<I> CanUseFooGetter<FooGetterComponent<I>, I> for MyContext {}
        impl<I> CanUseFooGetter<BarGetterComponent<I>, I> for MyContext {}
        ")
    }
}
