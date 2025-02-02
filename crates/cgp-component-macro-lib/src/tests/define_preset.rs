use quote::quote;

use crate::define_preset;
use crate::tests::helper::equal::equal_token_stream;

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

        pub trait IsFooPreset<Component> {}

        impl DelegateComponent<BarAComponent> for FooPreset {
            type Delegate = BazAComponents;
        }

        impl<__Context__, __Params__> IsProviderFor<BarAComponent, __Context__, __Params__> for FooPreset
        where
            BazAComponents: IsProviderFor<BarAComponent, __Context__, __Params__>,
        {}

        impl DelegateComponent<BarBComponent> for FooPreset {
            type Delegate = BazAComponents;
        }

        impl<__Context__, __Params__> IsProviderFor<BarBComponent, __Context__, __Params__> for FooPreset
        where
            BazAComponents: IsProviderFor<BarBComponent, __Context__, __Params__>,
        {}

        impl DelegateComponent<BarCComponent> for FooPreset {
            type Delegate = BazBComponents;
        }

        impl<__Context__, __Params__> IsProviderFor<BarCComponent, __Context__, __Params__> for FooPreset
        where
            BazBComponents: IsProviderFor<BarCComponent, __Context__, __Params__>,
        {}

        impl<T> IsFooPreset<BarAComponent> for T {}

        impl<T> IsFooPreset<BarBComponent> for T {}

        impl<T> IsFooPreset<BarCComponent> for T {}

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
                replace_with! {
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
                <'b, BarParamB: BarConstraint> BarComponentE<'b, BarParamB, FooParamB>,
            ]: BazComponentsB,
        }
    })
    .unwrap();

    let expected = quote! {
        pub struct FooPreset<'a, FooParamA, FooParamB>(
            pub ::core::marker::PhantomData<(&'a (), FooParamA, FooParamB)>,
        );

        pub trait IsFooPreset<Component> {}

        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentA>
            for FooPreset<'a, FooParamA, FooParamB>
        {
            type Delegate = BazComponentsA<FooParamA>;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint, __Context__, __Params__>
            IsProviderFor<BarComponentA, __Context__, __Params__>
            for FooPreset<'a, FooParamA, FooParamB>
        where
            BazComponentsA<FooParamA>: IsProviderFor<BarComponentA, __Context__, __Params__>,
        {}

        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentB<'a>>
            for FooPreset<'a, FooParamA, FooParamB>
        {
            type Delegate = BazComponentsB;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint, __Context__, __Params__>
            IsProviderFor<BarComponentB<'a>, __Context__, __Params__>
            for FooPreset<'a, FooParamA, FooParamB>
        where
            BazComponentsB: IsProviderFor<BarComponentB<'a>, __Context__, __Params__>,
        {}

        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentC<FooParamB>>
            for FooPreset<'a, FooParamA, FooParamB>
        {
            type Delegate = BazComponentsB;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint, __Context__, __Params__>
            IsProviderFor<BarComponentC<FooParamB>, __Context__, __Params__>
            for FooPreset<'a, FooParamA, FooParamB>
        where
            BazComponentsB: IsProviderFor<BarComponentC<FooParamB>, __Context__, __Params__>,
        {}

        impl<
            'a,
            FooParamA,
            FooParamB: FooConstraint,
            BarParamA,
        > DelegateComponent<BarComponentD<BarParamA, FooParamA>>
            for FooPreset<'a, FooParamA, FooParamB>
        {
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
            IsProviderFor<BarComponentD<BarParamA, FooParamA>, __Context__, __Params__>
            for FooPreset<'a, FooParamA, FooParamB>
        where
            BazComponentsB: IsProviderFor<BarComponentD<BarParamA, FooParamA>, __Context__, __Params__>,
        {}

        impl<
            'a,
            'b,
            FooParamA,
            FooParamB: FooConstraint,
            BarParamB: BarConstraint,
        > DelegateComponent<BarComponentE<'b, BarParamB, FooParamB>>
            for FooPreset<'a, FooParamA, FooParamB>
        {
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
            IsProviderFor<BarComponentE<'b, BarParamB, FooParamB>, __Context__, __Params__>
            for FooPreset<'a, FooParamA, FooParamB>
        where
            BazComponentsB: IsProviderFor<BarComponentE<'b, BarParamB, FooParamB>, __Context__, __Params__>,
        {}

        impl<T> IsFooPreset<BarComponentA> for T {}
        impl<T> IsFooPreset<BarComponentB<'a>> for T {}
        impl<T> IsFooPreset<BarComponentC<FooParamB>> for T {}

        impl<BarParamA, T> IsFooPreset<BarComponentD<BarParamA, FooParamA>> for T {}

        impl<'b, BarParamB: BarConstraint, T> IsFooPreset<BarComponentE<'b, BarParamB, FooParamB>> for T {}

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
                BarComponentE<'b, BarParamB, FooParamB>,
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
                    BarComponentE<'b, BarParamB, FooParamB>,
                    Delegate = FooPreset<'a, FooParamA, FooParamB>,
                >,
        {}

        #[macro_export]
        macro_rules! with_foo_preset {
            ($($body:tt)*) => {
                replace_with! {
                    [
                        BarComponentA,
                        BarComponentB<'a>,
                        BarComponentC<FooParamB>,
                        <BarParamA> BarComponentD<BarParamA, FooParamA>,
                        <'b, BarParamB: BarConstraint> BarComponentE<'b, BarParamB, FooParamB>
                    ],
                    $( $body )*
                }
            };
        }

        pub use with_foo_preset;
    };

    assert!(equal_token_stream(&derived, &expected));
}
