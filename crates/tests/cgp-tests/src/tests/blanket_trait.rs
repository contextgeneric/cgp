#![allow(dead_code)]

mod basic_blanket_trait {
    use cgp_macro_test_util::snapshot_blanket_trait;

    pub trait Foo {}
    pub trait Bar {}

    snapshot_blanket_trait! {
        #[blanket_trait]
        pub trait FooBar: Foo + Bar {}

        expand_foo_bar(output) {
            insta::assert_snapshot!(output, @"
            pub trait FooBar: Foo + Bar {}
            impl<__Context__> FooBar for __Context__
            where
                __Context__: Foo + Bar,
            {}
            ")
        }
    }

    pub struct Context;

    impl Foo for Context {}
    impl Bar for Context {}

    pub trait CanUseFooBar: FooBar {}
    impl CanUseFooBar for Context {}
}

mod blanket_trait_with_method {
    use cgp_macro_test_util::snapshot_blanket_trait;

    pub trait Foo {
        fn foo(&self);
    }
    pub trait Bar {
        fn bar(&self);
    }

    snapshot_blanket_trait! {
        #[blanket_trait]
        pub trait FooBar: Foo + Bar {
            fn foo_bar(&self) {
                self.foo();
                self.bar();
            }
        }

        expand_foo_bar(output) {
            insta::assert_snapshot!(output, @"
            pub trait FooBar: Foo + Bar {
                fn foo_bar(&self) {
                    self.foo();
                    self.bar();
                }
            }
            impl<__Context__> FooBar for __Context__
            where
                __Context__: Foo + Bar,
            {
                fn foo_bar(&self) {
                    self.foo();
                    self.bar();
                }
            }
            ")
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

mod blanket_trait_with_associated_type_without_constraints {
    use cgp_macro_test_util::snapshot_blanket_trait;

    pub trait HasFooTypeAt<I> {
        type Foo;
    }

    pub struct Bar;

    snapshot_blanket_trait! {
        #[blanket_trait]
        pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
            type FooBar;
        }

        expand_foo_bar(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
                type FooBar;
            }
            impl<__Context__, FooBar> HasFooTypeAtBar for __Context__
            where
                __Context__: HasFooTypeAt<Bar, Foo = FooBar>,
            {
                type FooBar = FooBar;
            }
            ")
        }
    }

    pub struct Context;
    pub struct FooBar;

    impl HasFooTypeAt<Bar> for Context {
        type Foo = FooBar;
    }

    pub trait CanUseFooTypeAtBar: HasFooTypeAtBar<FooBar = FooBar> {}
    impl CanUseFooTypeAtBar for Context {}
}

mod blanket_trait_with_associated_type_and_constraints {
    use cgp_macro_test_util::snapshot_blanket_trait;

    pub trait HasFooTypeAt<I> {
        type Foo;
    }

    pub struct Bar;

    snapshot_blanket_trait! {
        #[blanket_trait]
        pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
            type FooBar: Clone;
        }

        expand_foo_bar(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
                type FooBar: Clone;
            }
            impl<__Context__, FooBar> HasFooTypeAtBar for __Context__
            where
                __Context__: HasFooTypeAt<Bar, Foo = FooBar>,
                FooBar: Clone,
            {
                type FooBar = FooBar;
            }
            ")
        }
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
