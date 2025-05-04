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

    let name: &String =
        <ChainGetters<UseField<symbol!("inner")>, UseField<symbol!("name")>>>::get_field(
            &context,
            PhantomData::<()>,
        );
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

    let name: &String =
        <ChainGetters<UseField<symbol!("inner")>, UseField<symbol!("name")>>>::get_field(
            &context,
            PhantomData::<()>,
        );
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

    let name: &String =
        <ChainGetters<UseField<symbol!("inner")>, UseField<symbol!("name")>>>::get_field(
            &context,
            PhantomData::<()>,
        );
    assert_eq!(name, "test");
}
