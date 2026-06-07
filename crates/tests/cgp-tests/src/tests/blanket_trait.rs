#![allow(dead_code)]

use cgp::core::macros::blanket_trait;

#[test]
pub fn test_basic_blanket_trait() {
    pub trait Foo {}
    pub trait Bar {}

    #[blanket_trait]
    pub trait FooBar: Foo + Bar {}

    pub struct Context;

    impl Foo for Context {}
    impl Bar for Context {}

    pub trait CanUseFooBar: FooBar {}
    impl CanUseFooBar for Context {}
}

#[test]
pub fn test_blanket_trait_with_method() {
    pub trait Foo {
        fn foo(&self);
    }
    pub trait Bar {
        fn bar(&self);
    }

    #[blanket_trait]
    pub trait FooBar: Foo + Bar {
        fn foo_bar(&self) {
            self.foo();
            self.bar();
        }
    }

    pub struct Context;

    impl Foo for Context {
        fn foo(&self) {}
    }

    impl Bar for Context {
        fn bar(&self) {}
    }

    pub trait CanUseFooBar: FooBar {}
    impl CanUseFooBar for Context {}
}

#[test]
pub fn test_blanket_trait_with_associated_type_without_constraints() {
    pub trait HasFooTypeAt<I> {
        type Foo;
    }

    pub struct Bar;

    #[blanket_trait]
    pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
        type FooBar;
    }

    pub struct Context;
    pub struct FooBar;

    impl HasFooTypeAt<Bar> for Context {
        type Foo = FooBar;
    }

    pub trait CanUseFooTypeAtBar: HasFooTypeAtBar<FooBar = FooBar> {}
    impl CanUseFooTypeAtBar for Context {}
}

#[test]
pub fn test_blanket_trait_with_associated_type_and_constraints() {
    pub trait HasFooTypeAt<I> {
        type Foo;
    }

    pub struct Bar;

    #[blanket_trait]
    pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
        type FooBar: Clone;
    }

    pub struct Context;

    #[derive(Clone)]
    pub struct FooBar;

    impl HasFooTypeAt<Bar> for Context {
        type Foo = FooBar;
    }

    pub trait CanUseFooTypeAtBar: HasFooTypeAtBar<FooBar = FooBar> {}
    impl CanUseFooTypeAtBar for Context {}
}
