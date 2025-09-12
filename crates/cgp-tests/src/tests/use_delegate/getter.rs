use core::marker::PhantomData;

use cgp::prelude::*;

pub struct UseDelegate2<Components>(pub PhantomData<Components>);

#[cgp_type {
    provider: FooTypeProviderAt,
    derive_delegate: [
        UseDelegate<I>,
        UseDelegate2<(I, J)>,
    ],
}]
pub trait HasFooTypeAt<I, J> {
    type Foo;
}

#[cgp_getter {
    provider: FooGetterAt,
    derive_delegate: [
        UseDelegate<I>,
        UseDelegate2<(I, J)>,
    ],
}]
pub trait HasFooAt<I, J>: HasFooTypeAt<I, J> {
    fn foo_at(&self, _tag: PhantomData<(I, J)>) -> &Self::Foo;
}

#[test]
pub fn test_derive_delegate() {
    #[cgp_context]
    #[derive(HasField)]
    pub struct MyContext {
        pub foo: u64,
        pub bar: String,
    }

    delegate_components! {
        MyContextComponents {
            FooTypeProviderAtComponent: UseDelegate<
                new FooTypes {
                    Index<1>: UseType<u64>,
                    Index<0>: UseType<String>,
                }
            >,
            FooGetterAtComponent: UseDelegate<
                new FooGetters {
                    Index<1>: UseField<Symbol!("foo")>,
                    Index<0>: UseField<Symbol!("bar")>,
                }
            >
        }
    }

    check_components! {
        CanUseMyContext for MyContext {
            FooGetterAtComponent: [
                (Index<1>, Index<0>),
                (Index<0>, Index<1>),
            ]
        }
    }

    let context = MyContext {
        foo: 42,
        bar: "Bar".into(),
    };

    assert_eq!(context.foo_at(PhantomData::<(Index<1>, Index<0>)>), &42);
    assert_eq!(context.foo_at(PhantomData::<(Index<0>, Index<1>)>), "Bar");
}

#[test]
pub fn test_derive_delegate2() {
    #[cgp_context]
    #[derive(HasField)]
    pub struct MyContext {
        pub foo: u64,
        pub bar: String,
    }

    delegate_components! {
        MyContextComponents {
            FooTypeProviderAtComponent: UseDelegate2<
                new FooTypes {
                    (Index<1>, Index<0>): UseType<u64>,
                    (Index<0>, Index<1>): UseType<String>,
                }
            >,
            FooGetterAtComponent: UseDelegate2<
                new FooGetters {
                    (Index<1>, Index<0>): UseField<Symbol!("foo")>,
                    (Index<0>, Index<1>): UseField<Symbol!("bar")>,
                }
            >
        }
    }

    check_components! {
        CanUseMyContext for MyContext {
            FooGetterAtComponent: [
                (Index<1>, Index<0>),
                (Index<0>, Index<1>),
            ]
        }
    }

    let context = MyContext {
        foo: 42,
        bar: "Bar".into(),
    };

    assert_eq!(context.foo_at(PhantomData::<(Index<1>, Index<0>)>), &42);
    assert_eq!(context.foo_at(PhantomData::<(Index<0>, Index<1>)>), "Bar");
}
