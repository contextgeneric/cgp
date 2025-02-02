use quote::quote;

use crate::delegate_components;
use crate::tests::helper::equal::equal_token_stream;

#[test]
fn test_basic_delegate_components() {
    let derived = delegate_components(quote! {
        FooComponents {
            [
                BarAComponent,
                BarBComponent,
            ]: BazAComponents,
            BarCComponent: BazBComponents,
        }
    })
    .unwrap();

    let expected = quote! {
        impl DelegateComponent<BarAComponent> for FooComponents {
            type Delegate = BazAComponents;
        }

        impl<__Context__, __Params__> IsProvider<BarAComponent, __Context__, __Params__> for FooComponents
        where
            BazAComponents: IsProvider<BarAComponent, __Context__, __Params__>,
        {}

        impl DelegateComponent<BarBComponent> for FooComponents {
            type Delegate = BazAComponents;
        }

        impl<__Context__, __Params__> IsProvider<BarBComponent, __Context__, __Params__> for FooComponents
        where
            BazAComponents: IsProvider<BarBComponent, __Context__, __Params__>,
        {}

        impl DelegateComponent<BarCComponent> for FooComponents {
            type Delegate = BazBComponents;
        }

        impl<__Context__, __Params__> IsProvider<BarCComponent, __Context__, __Params__> for FooComponents
        where
            BazBComponents: IsProvider<BarCComponent, __Context__, __Params__>,
        {}
    };

    assert!(equal_token_stream(&derived, &expected));
}

#[test]
fn test_delegate_components_containing_generics() {
    let derived = delegate_components(quote! {
        <'a, FooParamA, FooParamB: FooConstraint>
        FooComponents<'a, FooParamA, FooParamB> {
            BarComponentA: BazComponentsA<FooParamA>,
            [
                BarComponentB<'a>,
                BarComponentC<FooParamB>,
                <BarParamA> BarComponentD<BarParamA, FooParamA>,
                <'b, BarParamB: BarConstraint> BarComponentE<BarParamB, FooParamB>,
            ]: BazComponentsB,
        }
    })
    .unwrap();

    let expected = quote! {
        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentA>
            for FooComponents<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsA<FooParamA>;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint, __Context__, __Params__>
            IsProvider<BarComponentA, __Context__, __Params__>
            for FooComponents<'a, FooParamA, FooParamB>
        where
            BazComponentsA<FooParamA>: IsProvider<BarComponentA, __Context__, __Params__>,
        {}

        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentB<'a>>
        for FooComponents<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsB;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint, __Context__, __Params__>
            IsProvider<BarComponentB<'a>, __Context__, __Params__>
            for FooComponents<'a, FooParamA, FooParamB>
        where
            BazComponentsB: IsProvider<BarComponentB<'a>, __Context__, __Params__>,
        {}

        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentC<FooParamB>>
        for FooComponents<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsB;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint, __Context__, __Params__>
            IsProvider<BarComponentC<FooParamB>, __Context__, __Params__>
            for FooComponents<'a, FooParamA, FooParamB>
        where
            BazComponentsB: IsProvider<BarComponentC<FooParamB>, __Context__, __Params__>,
        {}

        impl<
            'a,
            FooParamA,
            FooParamB: FooConstraint,
            BarParamA,
        > DelegateComponent<BarComponentD<BarParamA, FooParamA>>
        for FooComponents<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsB;
        }

        impl<
            'a,
            FooParamA,
            FooParamB: FooConstraint,
            BarParamA,
            __Context__,
            __Params__,
        >
            IsProvider<BarComponentD<BarParamA, FooParamA>, __Context__, __Params__>
            for FooComponents<'a, FooParamA, FooParamB>
        where
            BazComponentsB: IsProvider<BarComponentD<BarParamA, FooParamA>, __Context__, __Params__>,
        {}

        impl<
            'a,
            'b,
            FooParamA,
            FooParamB: FooConstraint,
            BarParamB: BarConstraint,
        > DelegateComponent<BarComponentE<BarParamB, FooParamB>>
        for FooComponents<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsB;
        }

        impl<
            'a,
            'b,
            FooParamA,
            FooParamB: FooConstraint,
            BarParamB: BarConstraint,
            __Context__,
            __Params__,
        >
            IsProvider<BarComponentE<BarParamB, FooParamB>, __Context__, __Params__>
            for FooComponents<'a, FooParamA, FooParamB>
        where
            BazComponentsB: IsProvider<BarComponentE<BarParamB, FooParamB>, __Context__, __Params__>,
        {}
    };

    assert!(equal_token_stream(&derived, &expected));
}
