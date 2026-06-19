use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_check_components, snapshot_delegate_components};

use crate::preset_tests::basic::components::{
    BarGetterComponent, BarTypeProviderComponent, FooGetterComponent, FooTypeProviderComponent,
};
use crate::preset_tests::basic::preset::{CheckDelegatesForMyPreset, MyPreset};

#[cgp_inherit(MyPreset)]
#[derive(HasField)]
pub struct MyContext {
    pub foo: (),
    pub bar: (),
}

snapshot_delegate_components! {
    delegate_components! {
        MyContext {
            BarGetterComponent: UseField<Symbol!("bar")>,
        }
    }

    expand_my_context(output) {
        insta::assert_snapshot!(output, @r#"
        impl DelegateComponent<BarGetterComponent> for MyContext {
            type Delegate = UseField<Symbol!("bar")>;
        }
        impl<__Context__, __Params__> IsProviderFor<BarGetterComponent, __Context__, __Params__>
        for MyContext
        where
            UseField<Symbol!("bar")>: IsProviderFor<BarGetterComponent, __Context__, __Params__>,
        {}
        "#)
    }
}

snapshot_check_components! {
    check_components! {
        MyContext {
            FooTypeProviderComponent,
            BarTypeProviderComponent,
            FooGetterComponent,
            BarGetterComponent,
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
        impl __CheckMyContext<FooGetterComponent, ()> for MyContext {}
        impl __CheckMyContext<BarGetterComponent, ()> for MyContext {}
        ")
    }
}

impl CheckDelegatesForMyPreset for MyContext {}
