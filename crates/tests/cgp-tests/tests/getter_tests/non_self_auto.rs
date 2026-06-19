use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_auto_getter, snapshot_delegate_components};

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

snapshot_delegate_components! {
    delegate_components! {
        App {
            FooTypeProviderComponent:
                UseType<Foo>,
            BarTypeProviderComponent:
                UseType<u32>,
        }
    }

    expand_app(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<FooTypeProviderComponent> for App {
            type Delegate = UseType<Foo>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<FooTypeProviderComponent, __Context__, __Params__> for App
        where
            UseType<Foo>: IsProviderFor<FooTypeProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<BarTypeProviderComponent> for App {
            type Delegate = UseType<u32>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<BarTypeProviderComponent, __Context__, __Params__> for App
        where
            UseType<u32>: IsProviderFor<BarTypeProviderComponent, __Context__, __Params__>,
        {}
        ")
    }
}

#[test]
fn test_non_self_getter() {
    let foo = Foo { foo_bar: 42 };

    let bar = App::foo_bar(&foo);
    assert_eq!(bar, &42);
}
