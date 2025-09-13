use cgp::core::field::impls::{CanDowncast, CanUpcast};
use cgp::prelude::*;

#[derive(Debug, Eq, PartialEq, CgpData)]
pub enum FooBarBazGeneric<Foo, Bar, Baz> {
    Foo(Foo),
    Bar(Bar),
    Baz(Baz),
}

#[derive(Debug, Eq, PartialEq, CgpData)]
pub enum FooBarGeneric<Foo, Bar> {
    Foo(Foo),
    Bar(Bar),
}

pub type FooBarBaz = FooBarBazGeneric<u64, String, bool>;
pub type FooBar = FooBarGeneric<u64, String>;

#[test]
fn test_upcast() {
    assert_eq!(
        FooBar::Foo(1).upcast(PhantomData::<FooBarBaz>),
        FooBarBaz::Foo(1)
    );

    assert_eq!(
        FooBar::Bar("hello".to_owned()).upcast(PhantomData::<FooBarBaz>),
        FooBarBaz::Bar("hello".to_owned())
    );
}

#[test]
fn test_downcast() {
    assert_eq!(
        FooBarBaz::Foo(1).downcast(PhantomData::<FooBar>).ok(),
        Some(FooBar::Foo(1))
    );

    assert_eq!(
        FooBarBaz::Bar("hello".to_owned())
            .downcast(PhantomData::<FooBar>)
            .ok(),
        Some(FooBar::Bar("hello".to_owned()))
    );

    assert_eq!(
        FooBarBaz::Baz(true).downcast(PhantomData::<FooBar>).ok(),
        None
    );
}
