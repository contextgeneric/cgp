#![allow(dead_code)]

use core::marker::PhantomData;

use cgp::prelude::*;

#[test]
pub fn test_basic_check_components() {
    #[cgp_type]
    pub trait HasFooType {
        type Foo;
    }

    #[cgp_type]
    pub trait HasBarType {
        type Bar;
    }

    #[cgp_getter {
        provider: FooGetterAt,
    }]
    pub trait HasFooAt<I>: HasFooType {
        fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo;
    }

    #[cgp_getter {
        provider: BarGetterAt,
    }]
    pub trait HasBarAt<I, J>: HasBarType {
        fn foo(&self, _tag: PhantomData<(I, J)>) -> &Self::Bar;
    }

    #[cgp_context]
    #[derive(HasField)]
    pub struct Context {
        pub dummy: (),
        pub extra_dummy: (),
    }

    delegate_components! {
        ContextComponents {
            [
                FooTypeProviderComponent,
                BarTypeProviderComponent,
            ]:
                UseType<()>,
            [
                FooGetterAtComponent,
                BarGetterAtComponent,
            ]:
                UseField<Symbol!("dummy")>,
        }
    }

    check_components! {
        CanUseContext for Context {
            FooTypeProviderComponent,
            BarTypeProviderComponent,
            FooGetterAtComponent: [
                Index<0>,
                Index<1>,
            ],
            FooGetterAtComponent:
                Index<3>,
        }

        CanUseContext2 for Context {
            BarGetterAtComponent: [
                (Index<0>, Index<1>),
                (Index<1>, Index<0>),
            ],
            BarGetterAtComponent:
                (Index<3>, Index<4>),
            [
                FooGetterAtComponent,
                BarGetterAtComponent,
            ]: [
                (Index<5>, Index<6>),
                (Index<7>, Index<8>),
            ]
        }

        #[check_providers(
            UseField<Symbol!("dummy")>,
            UseField<Symbol!("extra_dummy")>,
        )]
        CanUseDummyField for Context {
            FooGetterAtComponent: [
                Index<0>,
                Index<1>,
            ],
            FooGetterAtComponent:
                Index<3>,
            BarGetterAtComponent: [
                (Index<0>, Index<1>),
                (Index<1>, Index<0>),
            ],
            BarGetterAtComponent:
                (Index<3>, Index<4>),
            [
                FooGetterAtComponent,
                BarGetterAtComponent,
            ]: [
                (Index<5>, Index<6>),
                (Index<7>, Index<8>),
            ]
        }
    }
}

#[test]
pub fn test_generic_check_components() {
    #[cgp_type]
    pub trait HasFooType {
        type Foo;
    }

    #[cgp_type]
    pub trait HasBarType {
        type Bar;
    }

    #[cgp_getter {
        provider: FooGetterAt,
    }]
    pub trait HasFooAt<I: Clone>: HasFooType {
        fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo;
    }

    #[cgp_getter {
        name: BarGetterAtComponent<I>,
        provider: BarGetterAt,
    }]
    pub trait HasBarAt<I: Clone, J>: HasBarType {
        fn foo(&self, _tag: PhantomData<(I, J)>) -> &Self::Bar;
    }

    #[cgp_context]
    #[derive(HasField)]
    pub struct Context {
        pub dummy: (),
    }

    delegate_components! {
        ContextComponents {
            [
                FooTypeProviderComponent,
                BarTypeProviderComponent,
            ]:
                UseType<()>,
            [
                FooGetterAtComponent,
                <I> BarGetterAtComponent<I>,
            ]:
                UseField<Symbol!("dummy")>,
        }
    }

    check_components! {
        <'a, I>
        CanUseContext for Context
        where
            I: Clone,
        {
            FooGetterAtComponent: &'a I,
            BarGetterAtComponent<I>: (I, &'a Index<0>),
        }
    }
}
