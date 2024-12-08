use quote::quote;

use crate::define_preset;
use crate::tests::helper::equal::equal_token_stream;
use crate::tests::helper::format::format_token_stream;

#[test]
fn test_basic_define_preset() {
    let derived = define_preset(quote! {
        FooPreset {
            [
                BarAComponent,
                BarBComponent,
            ]: BazAComponents,
            BarCComponent: BazBComponents,
        }
    })
    .unwrap();

    let expected = quote! {
        pub struct FooPreset;

        pub trait IsFooPreset {}

        impl DelegateComponent<BarAComponent> for FooPreset {
            type Delegate = BazAComponents;
        }

        impl DelegateComponent<BarBComponent> for FooPreset {
            type Delegate = BazAComponents;
        }

        impl DelegateComponent<BarCComponent> for FooPreset {
            type Delegate = BazBComponents;
        }

        impl IsFooPreset for BarAComponent {}
        impl IsFooPreset for BarBComponent {}
        impl IsFooPreset for BarCComponent {}

        pub trait DelegatesToFooPreset: DelegateComponent<
                BarAComponent,
                Delegate = FooPreset,
            > + DelegateComponent<
                BarBComponent,
                Delegate = FooPreset,
            > + DelegateComponent<BarCComponent, Delegate = FooPreset> {}

        impl<Components> DelegatesToFooPreset for Components
        where
            Components: DelegateComponent<BarAComponent, Delegate = FooPreset>
                + DelegateComponent<BarBComponent, Delegate = FooPreset>
                + DelegateComponent<BarCComponent, Delegate = FooPreset>,
        {}

        #[macro_export]
        macro_rules! with_foo_preset {
            ($($body:tt)*) => {
                for_each_replace! {
                    [ BarAComponent, BarBComponent, BarCComponent ],
                    $( $body )*
                }
            };
        }

        pub use with_foo_preset;
    };

    assert!(equal_token_stream(&derived, &expected));
}

#[test]
fn test_define_preset_containing_generics() {
    let derived = define_preset(quote! {
        FooPreset<'a, FooParamA, FooParamB: FooConstraint> {
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
        pub struct FooPreset<'a, FooParamA, FooParamB>(
            pub ::core::marker::PhantomData<(&'a (), FooParamA, FooParamB)>,
        );

        pub trait IsFooPreset {}

        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentA>
        for FooPreset<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsA<FooParamA>;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentB<'a>>
        for FooPreset<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsB;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentC<FooParamB>>
        for FooPreset<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsB;
        }

        impl<
            'a,
            FooParamA,
            FooParamB: FooConstraint,
            BarParamA,
        > DelegateComponent<BarComponentD<BarParamA, FooParamA>>
        for FooPreset<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsB;
        }

        impl<
            'a,
            'b,
            FooParamA,
            FooParamB: FooConstraint,
            BarParamB: BarConstraint,
        > DelegateComponent<BarComponentE<BarParamB, FooParamB>>
        for FooPreset<'a, FooParamA, FooParamB> {
            type Delegate = BazComponentsB;
        }

        impl IsFooPreset for BarComponentA {}
        impl<'a> IsFooPreset for BarComponentB<'a> {}
        impl<FooParamB: FooConstraint> IsFooPreset for BarComponentC<FooParamB> {}
        impl<FooParamA, BarParamA> IsFooPreset for BarComponentD<BarParamA, FooParamA> {}
        impl<FooParamB: FooConstraint, BarParamB: BarConstraint> IsFooPreset for BarComponentE<BarParamB, FooParamB> {}

        pub trait DelegatesToFooPreset<
            'a,
            FooParamA,
            FooParamB: FooConstraint,
        >: DelegateComponent<
                BarComponentA,
                Delegate = FooPreset<'a, FooParamA, FooParamB>,
            > + DelegateComponent<
                BarComponentB<'a>,
                Delegate = FooPreset<'a, FooParamA, FooParamB>,
            > + DelegateComponent<
                BarComponentC<FooParamB>,
                Delegate = FooPreset<'a, FooParamA, FooParamB>,
            > + DelegateComponent<
                BarComponentD<BarParamA, FooParamA>,
                Delegate = FooPreset<'a, FooParamA, FooParamB>,
            > + DelegateComponent<
                BarComponentE<BarParamB, FooParamB>,
                Delegate = FooPreset<'a, FooParamA, FooParamB>,
            > {}

        impl<
            'a,
            FooParamA,
            FooParamB: FooConstraint,
            Components,
        > DelegatesToFooPreset<'a, FooParamA, FooParamB> for Components
        where
            Components: DelegateComponent<
                    BarComponentA,
                    Delegate = FooPreset<'a, FooParamA, FooParamB>,
                >
                + DelegateComponent<
                    BarComponentB<'a>,
                    Delegate = FooPreset<'a, FooParamA, FooParamB>,
                >
                + DelegateComponent<
                    BarComponentC<FooParamB>,
                    Delegate = FooPreset<'a, FooParamA, FooParamB>,
                >
                + DelegateComponent<
                    BarComponentD<BarParamA, FooParamA>,
                    Delegate = FooPreset<'a, FooParamA, FooParamB>,
                >
                + DelegateComponent<
                    BarComponentE<BarParamB, FooParamB>,
                    Delegate = FooPreset<'a, FooParamA, FooParamB>,
                >,
        {}

        #[macro_export]
        macro_rules! with_foo_preset {
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

        pub use with_foo_preset;
    };

    assert!(equal_token_stream(&derived, &expected));
}
