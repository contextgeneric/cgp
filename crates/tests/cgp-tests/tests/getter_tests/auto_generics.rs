use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasFoo<Foo> {
        fn foo(&self, _tag: PhantomData<Foo>) -> &Foo;
    }

    expand_has_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFoo<Foo> {
            fn foo(&self, _tag: PhantomData<Foo>) -> &Foo;
        }
        impl<__Context__, Foo> HasFoo<Foo> for __Context__
        where
            __Context__: HasField<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value = Foo,
            >,
        {
            fn foo(&self, _phantom: PhantomData<Foo>) -> &Foo {
                self.get_field(
                    ::core::marker::PhantomData::<
                        Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    >,
                )
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub foo: u32,
}

#[test]
fn test_generic_auto_getter() {
    let app = App { foo: 42 };

    assert_eq!(app.foo(PhantomData::<u32>), &42);
}
