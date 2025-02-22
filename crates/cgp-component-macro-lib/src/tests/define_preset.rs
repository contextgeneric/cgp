use quote::quote;

use crate::define_preset;
use crate::tests::helper::equal::assert_equal_token_stream;

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
        impl DelegateComponent<BarAComponent> for FooPreset::Provider {
            type Delegate = BazAComponents;
        }
        impl<__Context__, __Params__> IsProviderFor<BarAComponent, __Context__, __Params__>
            for FooPreset::Provider
        where
            BazAComponents: IsProviderFor<BarAComponent, __Context__, __Params__>,
        {
        }
        impl DelegateComponent<BarBComponent> for FooPreset::Provider {
            type Delegate = BazAComponents;
        }
        impl<__Context__, __Params__> IsProviderFor<BarBComponent, __Context__, __Params__>
            for FooPreset::Provider
        where
            BazAComponents: IsProviderFor<BarBComponent, __Context__, __Params__>,
        {
        }
        impl DelegateComponent<BarCComponent> for FooPreset::Provider {
            type Delegate = BazBComponents;
        }
        impl<__Context__, __Params__> IsProviderFor<BarCComponent, __Context__, __Params__>
            for FooPreset::Provider
        where
            BazBComponents: IsProviderFor<BarCComponent, __Context__, __Params__>,
        {
        }
        #[allow(non_snake_case)]
        pub mod FooPreset {
            use super::*;
            #[doc(hidden)]
            pub mod re_exports {
                #[doc(hidden)]
                #[doc(no_inline)]
                pub use super::super::super::re_exports::*;
            }
            pub struct Provider;
            #[doc(hidden)]
            pub trait IsPreset<Component> {}
            impl<T> IsPreset<BarAComponent> for T {}
            impl<T> IsPreset<BarBComponent> for T {}
            impl<T> IsPreset<BarCComponent> for T {}
            #[doc(hidden)]
            pub trait DelegatesToPreset:
                DelegateComponent<BarAComponent, Delegate = Provider>
                + DelegateComponent<BarBComponent, Delegate = Provider>
                + DelegateComponent<BarCComponent, Delegate = Provider>
            {
            }
            impl<Components> DelegatesToPreset for Components where
                Components: DelegateComponent<BarAComponent, Delegate = Provider>
                    + DelegateComponent<BarBComponent, Delegate = Provider>
                    + DelegateComponent<BarCComponent, Delegate = Provider>
            {
            }
            #[macro_export]
            #[doc(hidden)]
            macro_rules! with_foo_preset {
                    ($($body:tt)*) => {
                        replace_with! { [BarAComponent, BarBComponent, BarCComponent], $($body)* }
                    };
                }
            pub use with_foo_preset as with_components;
        }
    };

    assert_equal_token_stream(&derived, &expected);
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
        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentA>
            for FooPreset::Provider<'a, FooParamA, FooParamB>
        {
            type Delegate = BazComponentsA<FooParamA>;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint, __Context__, __Params__>
            IsProviderFor<BarComponentA, __Context__, __Params__>
            for FooPreset::Provider<'a, FooParamA, FooParamB>
        where
            BazComponentsA<FooParamA>: IsProviderFor<BarComponentA, __Context__, __Params__>,
        {
        }

        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentB<'a>>
            for FooPreset::Provider<'a, FooParamA, FooParamB>
        {
            type Delegate = BazComponentsB;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint, __Context__, __Params__>
            IsProviderFor<BarComponentB<'a>, __Context__, __Params__>
            for FooPreset::Provider<'a, FooParamA, FooParamB>
        where
            BazComponentsB: IsProviderFor<BarComponentB<'a>, __Context__, __Params__>,
        {
        }

        impl<'a, FooParamA, FooParamB: FooConstraint> DelegateComponent<BarComponentC<FooParamB>>
            for FooPreset::Provider<'a, FooParamA, FooParamB>
        {
            type Delegate = BazComponentsB;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint, __Context__, __Params__>
            IsProviderFor<BarComponentC<FooParamB>, __Context__, __Params__>
            for FooPreset::Provider<'a, FooParamA, FooParamB>
        where
            BazComponentsB: IsProviderFor<BarComponentC<FooParamB>, __Context__, __Params__>,
        {
        }

        impl<'a, FooParamA, FooParamB: FooConstraint, BarParamA>
            DelegateComponent<BarComponentD<BarParamA, FooParamA>>
            for FooPreset::Provider<'a, FooParamA, FooParamB>
        {
            type Delegate = BazComponentsB;
        }

        impl<'a, FooParamA, FooParamB: FooConstraint, BarParamA, __Context__, __Params__>
            IsProviderFor<BarComponentD<BarParamA, FooParamA>, __Context__, __Params__>
            for FooPreset::Provider<'a, FooParamA, FooParamB>
        where
            BazComponentsB: IsProviderFor<BarComponentD<BarParamA, FooParamA>, __Context__, __Params__>,
        {
        }

        impl<'a, 'b, FooParamA, FooParamB: FooConstraint, BarParamB: BarConstraint>
            DelegateComponent<BarComponentE<'b, BarParamB, FooParamB>>
            for FooPreset::Provider<'a, FooParamA, FooParamB>
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
            > IsProviderFor<BarComponentE<'b, BarParamB, FooParamB>, __Context__, __Params__>
            for FooPreset::Provider<'a, FooParamA, FooParamB>
        where
            BazComponentsB:
                IsProviderFor<BarComponentE<'b, BarParamB, FooParamB>, __Context__, __Params__>,
        {
        }

        #[allow(non_snake_case)]
        pub mod FooPreset {
            use super::*;

            #[doc(hidden)]
            pub mod re_exports {
                #[doc(hidden)]
                #[doc(no_inline)]
                pub use super::super::super::re_exports::*;
            }

            pub struct Provider<'a, FooParamA, FooParamB>(
                pub ::core::marker::PhantomData<(&'a (), FooParamA, FooParamB)>,
            );

            #[doc(hidden)]
            pub trait IsPreset<Component> {}

            impl<T> IsPreset<BarComponentA> for T {}
            impl<T> IsPreset<BarComponentB<'a>> for T {}
            impl<T> IsPreset<BarComponentC<FooParamB>> for T {}
            impl<BarParamA, T> IsPreset<BarComponentD<BarParamA, FooParamA>> for T {}
            impl<'b, BarParamB: BarConstraint, T> IsPreset<BarComponentE<'b, BarParamB, FooParamB>> for T {}

            #[doc(hidden)]
            pub trait DelegatesToPreset<'a, FooParamA, FooParamB: FooConstraint>:
                DelegateComponent<BarComponentA, Delegate = Provider<'a, FooParamA, FooParamB>>
                + DelegateComponent<BarComponentB<'a>, Delegate = Provider<'a, FooParamA, FooParamB>>
                + DelegateComponent<
                    BarComponentC<FooParamB>,
                    Delegate = Provider<'a, FooParamA, FooParamB>,
                > + DelegateComponent<
                    BarComponentD<BarParamA, FooParamA>,
                    Delegate = Provider<'a, FooParamA, FooParamB>,
                > + DelegateComponent<
                    BarComponentE<'b, BarParamB, FooParamB>,
                    Delegate = Provider<'a, FooParamA, FooParamB>,
                >
            {
            }

            impl<'a, FooParamA, FooParamB: FooConstraint, Components>
                DelegatesToPreset<'a, FooParamA, FooParamB> for Components
            where
                Components: DelegateComponent<BarComponentA, Delegate = Provider<'a, FooParamA, FooParamB>>
                    + DelegateComponent<BarComponentB<'a>, Delegate = Provider<'a, FooParamA, FooParamB>>
                    + DelegateComponent<
                        BarComponentC<FooParamB>,
                        Delegate = Provider<'a, FooParamA, FooParamB>,
                    > + DelegateComponent<
                        BarComponentD<BarParamA, FooParamA>,
                        Delegate = Provider<'a, FooParamA, FooParamB>,
                    > + DelegateComponent<
                        BarComponentE<'b, BarParamB, FooParamB>,
                        Delegate = Provider<'a, FooParamA, FooParamB>,
                    >,
            {
            }

            #[macro_export]
            #[doc(hidden)]
            macro_rules! with_foo_preset {
                ($($body:tt)*) => {
                    replace_with! { [BarComponentA, BarComponentB < 'a >, BarComponentC <
                    FooParamB >, < BarParamA > BarComponentD < BarParamA, FooParamA >, < 'b,
                    BarParamB : BarConstraint > BarComponentE < 'b, BarParamB, FooParamB >],
                    $($body)* }
                };
            }

            pub use with_foo_preset as with_components;
        }
    };

    assert_equal_token_stream(&derived, &expected);
}
