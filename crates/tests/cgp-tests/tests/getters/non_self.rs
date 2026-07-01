//! `#[cgp_getter]` on a non-`self` getter (`fn foo_bar(foo: &Self::Foo) -> &Self::Bar`):
//! the getter reads a field *of another type* (`Self::Foo`) rather than of the
//! context, so `App::foo_bar` fetches the `bar` field out of a `Foo` value. The
//! abstract types and the `delegate_components!` wiring are written plainly here
//! (their expansions are owned by the `abstract_types` and `basic_delegation`
//! concepts).
//!
//! See docs/reference/macros/cgp_getter.md and docs/reference/providers/use_field.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_getter;

#[cgp_type]
pub trait HasFooType {
    type Foo;
}

#[cgp_type]
pub trait HasBarType {
    type Bar;
}

snapshot_cgp_getter! {
    #[cgp_getter]
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
            __Context__: FooBarGetter<__Context__>,
        {
            fn foo_bar(foo: &Self::Foo) -> &Self::Bar {
                __Context__::foo_bar(foo)
            }
        }
        pub trait FooBarGetter<
            __Context__,
        >: IsProviderFor<FooBarGetterComponent, __Context__, ()>
        where
            __Context__: HasFooType + HasBarType,
        {
            fn foo_bar(foo: &__Context__::Foo) -> &__Context__::Bar;
        }
        impl<__Provider__, __Context__> FooBarGetter<__Context__> for __Provider__
        where
            __Context__: HasFooType + HasBarType,
            __Provider__: DelegateComponent<FooBarGetterComponent>
                + IsProviderFor<FooBarGetterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                FooBarGetterComponent,
            >>::Delegate: FooBarGetter<__Context__>,
        {
            fn foo_bar(foo: &__Context__::Foo) -> &__Context__::Bar {
                <__Provider__ as DelegateComponent<
                    FooBarGetterComponent,
                >>::Delegate::foo_bar(foo)
            }
        }
        pub struct FooBarGetterComponent;
        impl<__Context__> FooBarGetter<__Context__> for UseContext
        where
            __Context__: HasFooType + HasBarType,
            __Context__: HasFooBar,
        {
            fn foo_bar(foo: &__Context__::Foo) -> &__Context__::Bar {
                __Context__::foo_bar(foo)
            }
        }
        impl<__Context__> IsProviderFor<FooBarGetterComponent, __Context__, ()> for UseContext
        where
            __Context__: HasFooType + HasBarType,
            __Context__: HasFooBar,
        {}
        impl<__Context__, __Components__, __Path__> FooBarGetter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooType + HasBarType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: FooBarGetter<__Context__>,
        {
            fn foo_bar(foo: &__Context__::Foo) -> &__Context__::Bar {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::foo_bar(foo)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<FooBarGetterComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooType + HasBarType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<FooBarGetterComponent, __Context__, ()>
                + FooBarGetter<__Context__>,
        {}
        impl<__Context__> FooBarGetter<__Context__> for UseFields
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
        impl<__Context__> IsProviderFor<FooBarGetterComponent, __Context__, ()> for UseFields
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
        {}
        impl<__Context__, __Tag__> FooBarGetter<__Context__> for UseField<__Tag__>
        where
            __Context__: HasFooType + HasBarType,
            __Context__::Foo: HasField<__Tag__, Value = __Context__::Bar>,
        {
            fn foo_bar(__context__: &__Context__::Foo) -> &__Context__::Bar {
                __context__.get_field(::core::marker::PhantomData::<__Tag__>)
            }
        }
        impl<__Context__, __Tag__> IsProviderFor<FooBarGetterComponent, __Context__, ()>
        for UseField<__Tag__>
        where
            __Context__: HasFooType + HasBarType,
            __Context__::Foo: HasField<__Tag__, Value = __Context__::Bar>,
        {}
        impl<__Context__, __Provider__> FooBarGetter<__Context__> for WithProvider<__Provider__>
        where
            __Context__: HasFooType + HasBarType,
            __Provider__: FieldGetter<
                __Context__::Foo,
                FooBarGetterComponent,
                Value = __Context__::Bar,
            >,
        {
            fn foo_bar(__context__: &__Context__::Foo) -> &__Context__::Bar {
                __Provider__::get_field(
                    __context__,
                    ::core::marker::PhantomData::<FooBarGetterComponent>,
                )
            }
        }
        impl<__Context__, __Provider__> IsProviderFor<FooBarGetterComponent, __Context__, ()>
        for WithProvider<__Provider__>
        where
            __Context__: HasFooType + HasBarType,
            __Provider__: FieldGetter<
                __Context__::Foo,
                FooBarGetterComponent,
                Value = __Context__::Bar,
            >,
        {}
        ")
    }
}

pub struct App;

#[derive(HasField)]
pub struct Foo {
    pub bar: u32,
}

delegate_components! {
    App {
        FooTypeProviderComponent:
            UseType<Foo>,
        BarTypeProviderComponent:
            UseType<u32>,
        FooBarGetterComponent:
            UseField<Symbol!("bar")>,
    }
}

#[test]
fn test_non_self_getter() {
    let foo = Foo { bar: 42 };

    let bar = <App as HasFooBar>::foo_bar(&foo);
    assert_eq!(bar, &42);
}
