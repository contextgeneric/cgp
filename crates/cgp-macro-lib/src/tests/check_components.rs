use quote::quote;

use crate::check_components;
use crate::tests::helper::equal::assert_equal_token_stream;

#[test]
pub fn test_basic_check_components() {
    let derived = check_components(quote! {
        CanUseMyApp for MyApp
        {
            FooAComponent,
            FooBComponent,
            [
                BarAComponent,
                BarBComponent,
            ]:
                (BarAParam, BarBParam),
            BazComponent: [
                BazAParam,
                BazBParam,
            ],
            [
                QuuxAComponent,
                QuuxBComponent,
            ]: [
                QuuxAParam,
                QuuxBParam,
            ]
        }
    })
    .unwrap();

    let expected = quote! {
        trait CanUseMyApp<Component, Params>: CanUseComponent<Component, Params> {}
        impl CanUseMyApp<FooAComponent, ()> for MyApp {}
        impl CanUseMyApp<FooBComponent, ()> for MyApp {}

        impl CanUseMyApp<BarAComponent, (BarAParam, BarBParam)> for MyApp {}
        impl CanUseMyApp<BarBComponent, (BarAParam, BarBParam)> for MyApp {}

        impl CanUseMyApp<BazComponent, BazAParam> for MyApp {}
        impl CanUseMyApp<BazComponent, BazBParam> for MyApp {}

        impl CanUseMyApp<QuuxAComponent, QuuxAParam> for MyApp {}
        impl CanUseMyApp<QuuxAComponent, QuuxBParam> for MyApp {}
        impl CanUseMyApp<QuuxBComponent, QuuxAParam> for MyApp {}
        impl CanUseMyApp<QuuxBComponent, QuuxBParam> for MyApp {}
    };

    assert_equal_token_stream(&derived, &expected);
}

#[test]
pub fn test_generic_check_components() {
    let derived = check_components(quote! {
        <'a, T>
        CanUseMyAppWithT for MyApp<T>
        where
            T: Clone,
        {
            FooComponent: FooParam<'a, T>,
            BarComponent<T>: BarParam<'a>,
        }
    })
    .unwrap();

    let expected = quote! {
        trait CanUseMyAppWithT<Component, Params>: CanUseComponent<Component, Params> {}

        impl<'a, T> CanUseMyAppWithT<FooComponent, FooParam<'a, T>> for MyApp<T>
        where
            T: Clone,
        {}

        impl<'a, T> CanUseMyAppWithT<BarComponent<T>, BarParam<'a>> for MyApp<T>
        where
            T: Clone,
        {}
    };

    assert_equal_token_stream(&derived, &expected);
}
