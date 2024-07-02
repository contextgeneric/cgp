use quote::quote;

use crate::delegate_components::define_components;
use crate::tests::helper::equal::equal_token_stream;

#[test]
fn test_basic_define_components() {
    let derived = define_components(quote! {
        FooComponents {
            [
                BarAComponent,
                BarBComponent,
            ]: BazAComponents,
            BarCComponent: BazBComponents,
        }
    });

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
            (@ remaining() @ out($($out:tt)*) @ stack()) => {
                $($out)*
            };
            (
                @ remaining() @ out($($out:tt)*) @ stack(@ layer { @ front($($front:tt)*) @
                remaining($($remaining:tt)*) } $($stack:tt)*)
            ) => {
                $crate ::with_foo_components! { @ remaining($($remaining)*) @ out($($front)* {
                $($out)* }) @ stack($($stack)*) }
            };
            (
                @ remaining() @ out($($out:tt)*) @ stack(@ layer[@ front($($front:tt)*) @
                remaining($($remaining:tt)*)] $($stack:tt)*)
            ) => {
                $crate ::with_foo_components! { @ remaining($($remaining)*) @ out($($front)*
                [$($out)*]) @ stack($($stack)*) }
            };
            (
                @ remaining(@ FooComponents $($remaining:tt)*) @ out($($out:tt)*) @
                stack($($stack:tt)*)
            ) => {
                $crate ::with_foo_components! { @ remaining($($remaining)*) @ out($($out)*
                [BarAComponent, BarBComponent, BarCComponent]) @ stack($($stack)*) }
            };
            (
                @ remaining({ $($inner:tt)* } $($outer:tt)*) @ out($($out:tt)*) @
                stack($($stack:tt)*)
            ) => {
                $crate ::with_foo_components! { @ remaining($($inner)*) @ out() @ stack(@ layer {
                @ front($($out)*) @ remaining($($outer)*) } $($stack)*) }
            };
            (
                @ remaining([$($inner:tt)*] $($outer:tt)*) @ out($($out:tt)*) @
                stack($($stack:tt)*)
            ) => {
                $crate ::with_foo_components! { @ remaining($($inner)*) @ out() @ stack(@ layer[@
                front($($out)*) @ remaining($($outer)*)] $($stack)*) }
            };
            (
                @ remaining($current:tt $($remaining:tt)*) @ out($($out:tt)*) @
                stack($($stack:tt)*)
            ) => {
                $crate ::with_foo_components! { @ remaining($($remaining)*) @ out($($out)*
                $current) @ stack($($stack)*) }
            };
            ($($remaining:tt)*) => {
                $crate ::with_foo_components! { @ remaining($($remaining)*) @ out() @ stack() }
            };
        }

        pub use with_foo_components;
    };

    assert!(equal_token_stream(&derived, &expected));
}

#[test]
fn test_define_components_containing_generics() {
    let derived = define_components(quote! {
        FooComponents<'a, FooParamA, FooParamB: FooConstraint> {
            BarComponentA: BazComponentsA<FooParamA>,
            [
                BarComponentB<'a>,
                BarComponentC<FooParamB>,
                <BarParamA> BarComponentD<BarParamA, FooParamA>,
                <'b, BarParamB: BarConstraint> BarComponentE<BarParamB, FooParamB>,
            ]: BazComponentsB,
        }
    });

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
            (@ remaining() @ out($($out:tt)*) @ stack()) => {
                $($out)*
            };
            (
                @ remaining() @ out($($out:tt)*) @ stack(@ layer { @ front($($front:tt)*) @
                remaining($($remaining:tt)*) } $($stack:tt)*)
            ) => {
                $crate ::with_foo_components! { @ remaining($($remaining)*) @ out($($front)* {
                $($out)* }) @ stack($($stack)*) }
            };
            (
                @ remaining() @ out($($out:tt)*) @ stack(@ layer[@ front($($front:tt)*) @
                remaining($($remaining:tt)*)] $($stack:tt)*)
            ) => {
                $crate ::with_foo_components! { @ remaining($($remaining)*) @ out($($front)*
                [$($out)*]) @ stack($($stack)*) }
            };
            (
                @ remaining(@ FooComponents $($remaining:tt)*) @ out($($out:tt)*) @
                stack($($stack:tt)*)
            ) => {
                $crate ::with_foo_components! { @ remaining($($remaining)*) @ out($($out)*
                [BarComponentA, BarComponentB < 'a >, BarComponentC < FooParamB >, < BarParamA >
                BarComponentD < BarParamA, FooParamA >, < 'b, BarParamB : BarConstraint >
                BarComponentE < BarParamB, FooParamB >]) @ stack($($stack)*) }
            };
            (
                @ remaining({ $($inner:tt)* } $($outer:tt)*) @ out($($out:tt)*) @
                stack($($stack:tt)*)
            ) => {
                $crate ::with_foo_components! { @ remaining($($inner)*) @ out() @ stack(@ layer {
                @ front($($out)*) @ remaining($($outer)*) } $($stack)*) }
            };
            (
                @ remaining([$($inner:tt)*] $($outer:tt)*) @ out($($out:tt)*) @
                stack($($stack:tt)*)
            ) => {
                $crate ::with_foo_components! { @ remaining($($inner)*) @ out() @ stack(@ layer[@
                front($($out)*) @ remaining($($outer)*)] $($stack)*) }
            };
            (
                @ remaining($current:tt $($remaining:tt)*) @ out($($out:tt)*) @
                stack($($stack:tt)*)
            ) => {
                $crate ::with_foo_components! { @ remaining($($remaining)*) @ out($($out)*
                $current) @ stack($($stack)*) }
            };
            ($($remaining:tt)*) => {
                $crate ::with_foo_components! { @ remaining($($remaining)*) @ out() @ stack() }
            };
        }

        pub use with_foo_components;
    };

    assert!(equal_token_stream(&derived, &expected));
}
