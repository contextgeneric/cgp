//! `#[cgp_auto_getter]` on a non-`self` getter
//! (`fn foo_bar(foo: &Self::Foo) -> &Self::Bar`): the blanket impl reads a field
//! *of another type* (`Self::Foo`) named after the method, so `App::foo_bar`
//! fetches the `foo_bar` field out of a `Foo` value. The abstract types and the
//! `delegate_components!` wiring are written plainly here (their expansions are
//! owned by the `abstract_types` and `basic_delegation` concepts).
//!
//! See docs/reference/macros/cgp_auto_getter.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

#[cgp_type]
pub trait HasFooType {
    type Foo;
}

#[cgp_type]
pub trait HasBarType {
    type Bar;
}

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasFooBar: HasFooType + HasBarType {
        fn foo_bar(foo: &Self::Foo) -> &Self::Bar;
    }

    expand_has_foo_bar(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFooBar: HasFooType + HasBarType {
            fn foo_bar(foo: &Self::Foo) -> &Self::Bar;
        }
        impl<__Context__> HasFooBar for __Context__
        where
            __Context__: HasFooType + HasBarType,
            __Context__::Foo: HasField<
                Symbol<
                    7,
                    Chars<
                        'f',
                        Chars<
                            'o',
                            Chars<'o', Chars<'_', Chars<'b', Chars<'a', Chars<'r', Nil>>>>>,
                        >,
                    >,
                >,
                Value = __Context__::Bar,
            >,
        {
            fn foo_bar(__context__: &__Context__::Foo) -> &__Context__::Bar {
                __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                7,
                                Chars<
                                    'f',
                                    Chars<
                                        'o',
                                        Chars<
                                            'o',
                                            Chars<'_', Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    )
            }
        }
        ")
    }
}

pub struct App;

#[derive(HasField)]
pub struct Foo {
    pub foo_bar: u32,
}

delegate_components! {
    App {
        FooTypeProviderComponent:
            UseType<Foo>,
        BarTypeProviderComponent:
            UseType<u32>,
    }
}

#[test]
fn test_non_self_getter() {
    let foo = Foo { foo_bar: 42 };

    let bar = App::foo_bar(&foo);
    assert_eq!(bar, &42);
}
