use core::marker::PhantomData;

use cgp::core::field::ChainGetters;
use cgp::prelude::*;

#[test]
fn test_chained_getter() {
    #[derive(HasField)]
    pub struct Outer {
        pub inner: Inner,
    }

    #[derive(HasField)]
    pub struct Inner {
        pub name: String,
    }

    let context = Outer {
        inner: Inner {
            name: "test".to_owned(),
        },
    };

    let name: &String = <ChainGetters<
        Product![UseField<symbol!("inner")>, UseField<symbol!("name")>],
    >>::get_field(&context, PhantomData::<()>);
    assert_eq!(name, "test");
}

#[test]
fn test_chained_getter_with_outer_life() {
    #[derive(HasField)]
    pub struct Outer<'a> {
        pub inner: &'a Inner,
    }

    #[derive(HasField)]
    pub struct Inner {
        pub name: String,
    }

    let context = Outer {
        inner: &Inner {
            name: "test".to_owned(),
        },
    };

    let name: &String = <ChainGetters<
        Product![UseField<symbol!("inner")>, UseField<symbol!("name")>],
    >>::get_field(&context, PhantomData::<()>);
    assert_eq!(name, "test");
}

#[test]
fn test_chained_getter_with_inner_life() {
    #[derive(HasField)]
    pub struct Outer<'a> {
        pub inner: Inner<'a>,
    }

    #[derive(HasField)]
    pub struct Inner<'a> {
        pub name: &'a String,
    }

    let context = Outer {
        inner: Inner {
            name: &"test".to_owned(),
        },
    };

    let name: &String = <ChainGetters<
        Product![UseField<symbol!("inner")>, UseField<symbol!("name")>],
    >>::get_field(&context, PhantomData::<()>);

    assert_eq!(name, "test");
}

#[test]
fn test_deeply_nested_getter() {
    #[derive(HasField)]
    pub struct A {
        pub b: B,
    }

    #[derive(HasField)]
    pub struct B {
        pub c: C,
    }

    #[derive(HasField)]
    pub struct C {
        pub d: D,
    }

    #[derive(HasField)]
    pub struct D {
        pub name: String,
    }

    #[cgp_context]
    #[derive(HasField)]
    pub struct MyContext {
        pub a: A,
    }

    #[cgp_getter]
    pub trait HasName {
        fn name(&self) -> &str;
    }

    delegate_and_check_components! {
        CanUseMyContext for MyContext;
        MyContextComponents {
            NameGetterComponent: WithProvider<
                ChainGetters<Product![
                    UseField<symbol!("a")>,
                    UseField<symbol!("b")>,
                    UseField<symbol!("c")>,
                    UseField<symbol!("d")>,
                    UseField<symbol!("name")>
                ]>>
        }
    }

    let context = MyContext {
        a: A {
            b: B {
                c: C {
                    d: D {
                        name: "test".to_owned(),
                    },
                },
            },
        },
    };

    assert_eq!(context.name(), "test");
}
