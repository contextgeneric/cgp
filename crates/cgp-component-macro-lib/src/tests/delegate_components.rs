use quote::quote;

use crate::delegate_components;
use crate::tests::helper::equal::equal_token_stream;
use crate::tests::helper::format::format_token_stream;

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

        impl DelegateComponent<BarBComponent> for FooComponents {
            type Delegate = BazAComponents;
        }

        impl DelegateComponent<BarCComponent> for FooComponents {
            type Delegate = BazBComponents;
        }
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

    println!("derived: {}", format_token_stream(&derived));

    let expected = quote! {
        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentA>
        for FooComponents<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsA<FooParamA>;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentB<'a>>
        for FooComponents<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsB;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentC<FooParamB>>
        for FooComponents<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsB;
        }

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
            'b,
            FooParamA,
            FooParamB: FooConstraint,
            BarParamB: BarConstraint,
        > DelegateComponent<BarComponentE<BarParamB, FooParamB>>
        for FooComponents<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsB;
        }
    };

    assert!(equal_token_stream(&derived, &expected));
}
