use quote::quote;

use crate::define_preset;
use crate::tests::helper::equal::equal_token_stream;

#[test]
fn test_basic_define_preset() {
    let derived = define_preset(quote! {
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
        pub struct FooComponents;

        impl DelegateComponent<BarAComponent> for FooComponents {
            type Delegate = BazAComponents;
        }

        impl DelegateComponent<BarBComponent> for FooComponents {
            type Delegate = BazAComponents;
        }

        impl DelegateComponent<BarCComponent> for FooComponents {
            type Delegate = BazBComponents;
        }

        pub trait DelegatesToFooComponents: DelegateComponent<
                BarAComponent,
                Delegate = FooComponents,
            > + DelegateComponent<
                BarBComponent,
                Delegate = FooComponents,
            > + DelegateComponent<BarCComponent, Delegate = FooComponents> {}

        impl<Components> DelegatesToFooComponents for Components
        where
            Components: DelegateComponent<BarAComponent, Delegate = FooComponents>
                + DelegateComponent<BarBComponent, Delegate = FooComponents>
                + DelegateComponent<BarCComponent, Delegate = FooComponents>,
        {}

        #[macro_export]
        macro_rules! with_foo_components {
            ($($body:tt)*) => {
                for_each_replace! {
                    [ BarAComponent, BarBComponent, BarCComponent ],
                    $( $body )*
                }
            };
        }

        pub use with_foo_components;
    };

    assert!(equal_token_stream(&derived, &expected));
}

#[test]
fn test_define_preset_containing_generics() {
    let derived = define_preset(quote! {
        FooComponents<'a, FooParamA, FooParamB: FooConstraint> {
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
        pub struct FooComponents<'a, FooParamA, FooParamB>(
            pub ::core::marker::PhantomData<(&'a (), FooParamA, FooParamB)>,
        );

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

        pub trait DelegatesToFooComponents<
            'a,
            FooParamA,
            FooParamB: FooConstraint,
        >: DelegateComponent<
                BarComponentA,
                Delegate = FooComponents<'a, FooParamA, FooParamB>,
            > + DelegateComponent<
                BarComponentB<'a>,
                Delegate = FooComponents<'a, FooParamA, FooParamB>,
            > + DelegateComponent<
                BarComponentC<FooParamB>,
                Delegate = FooComponents<'a, FooParamA, FooParamB>,
            > + DelegateComponent<
                BarComponentD<BarParamA, FooParamA>,
                Delegate = FooComponents<'a, FooParamA, FooParamB>,
            > + DelegateComponent<
                BarComponentE<BarParamB, FooParamB>,
                Delegate = FooComponents<'a, FooParamA, FooParamB>,
            > {}

        impl<
            'a,
            FooParamA,
            FooParamB: FooConstraint,
            Components,
        > DelegatesToFooComponents<'a, FooParamA, FooParamB> for Components
        where
            Components: DelegateComponent<
                    BarComponentA,
                    Delegate = FooComponents<'a, FooParamA, FooParamB>,
                >
                + DelegateComponent<
                    BarComponentB<'a>,
                    Delegate = FooComponents<'a, FooParamA, FooParamB>,
                >
                + DelegateComponent<
                    BarComponentC<FooParamB>,
                    Delegate = FooComponents<'a, FooParamA, FooParamB>,
                >
                + DelegateComponent<
                    BarComponentD<BarParamA, FooParamA>,
                    Delegate = FooComponents<'a, FooParamA, FooParamB>,
                >
                + DelegateComponent<
                    BarComponentE<BarParamB, FooParamB>,
                    Delegate = FooComponents<'a, FooParamA, FooParamB>,
                >,
        {}

        #[macro_export]
        macro_rules! with_foo_components {
            ($($body:tt)*) => {
                for_each_replace! {
                    [
                        BarComponentA,
                        BarComponentB<'a>,
                        BarComponentC<FooParamB>,
                        <BarParamA> BarComponentD<BarParamA, FooParamA>,
                        <'b, BarParamB: BarConstraint> BarComponentE<BarParamB, FooParamB>
                    ],
                    $( $body )*
                }
            };
        }

        pub use with_foo_components;
    };

    assert!(equal_token_stream(&derived, &expected));
}
