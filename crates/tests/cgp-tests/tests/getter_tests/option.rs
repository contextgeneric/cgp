mod option_getter {
    use cgp::prelude::*;
    use cgp_macro_test_util::{snapshot_cgp_getter, snapshot_delegate_components};

    snapshot_cgp_getter! {
        #[cgp_getter]
        pub trait HasFoo {
            fn foo(&self) -> Option<&String>;
        }

        expand_has_foo(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasFoo {
                fn foo(&self) -> Option<&String>;
            }
            impl<__Context__> HasFoo for __Context__
            where
                __Context__: FooGetter<__Context__>,
            {
                fn foo(&self) -> Option<&String> {
                    __Context__::foo(self)
                }
            }
            pub trait FooGetter<__Context__>: IsProviderFor<FooGetterComponent, __Context__, ()> {
                fn foo(__context__: &__Context__) -> Option<&String>;
            }
            impl<__Provider__, __Context__> FooGetter<__Context__> for __Provider__
            where
                __Provider__: DelegateComponent<FooGetterComponent>
                    + IsProviderFor<FooGetterComponent, __Context__, ()>,
                <__Provider__ as DelegateComponent<
                    FooGetterComponent,
                >>::Delegate: FooGetter<__Context__>,
            {
                fn foo(__context__: &__Context__) -> Option<&String> {
                    <__Provider__ as DelegateComponent<
                        FooGetterComponent,
                    >>::Delegate::foo(__context__)
                }
            }
            pub struct FooGetterComponent;
            impl<__Context__> FooGetter<__Context__> for UseContext
            where
                __Context__: HasFoo,
            {
                fn foo(__context__: &__Context__) -> Option<&String> {
                    __Context__::foo(__context__)
                }
            }
            impl<__Context__> IsProviderFor<FooGetterComponent, __Context__, ()> for UseContext
            where
                __Context__: HasFoo,
            {}
            impl<__Context__, __Components__, __Path__> FooGetter<__Context__>
            for RedirectLookup<__Components__, __Path__>
            where
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<__Path__>>::Delegate: FooGetter<__Context__>,
            {
                fn foo(__context__: &__Context__) -> Option<&String> {
                    <__Components__ as DelegateComponent<__Path__>>::Delegate::foo(__context__)
                }
            }
            impl<
                __Context__,
                __Components__,
                __Path__,
            > IsProviderFor<FooGetterComponent, __Context__, ()>
            for RedirectLookup<__Components__, __Path__>
            where
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate: IsProviderFor<FooGetterComponent, __Context__, ()>
                    + FooGetter<__Context__>,
            {}
            impl<__Context__> FooGetter<__Context__> for UseFields
            where
                __Context__: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value = Option<String>,
                >,
            {
                fn foo(__context__: &__Context__) -> Option<&String> {
                    __context__
                        .get_field(
                            ::core::marker::PhantomData::<
                                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                            >,
                        )
                        .as_ref()
                }
            }
            impl<__Context__> IsProviderFor<FooGetterComponent, __Context__, ()> for UseFields
            where
                __Context__: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value = Option<String>,
                >,
            {}
            impl<__Context__, __Tag__> FooGetter<__Context__> for UseField<__Tag__>
            where
                __Context__: HasField<__Tag__, Value = Option<String>>,
            {
                fn foo(__context__: &__Context__) -> Option<&String> {
                    __context__.get_field(::core::marker::PhantomData::<__Tag__>).as_ref()
                }
            }
            impl<__Context__, __Tag__> IsProviderFor<FooGetterComponent, __Context__, ()>
            for UseField<__Tag__>
            where
                __Context__: HasField<__Tag__, Value = Option<String>>,
            {}
            impl<__Context__, __Provider__> FooGetter<__Context__> for WithProvider<__Provider__>
            where
                __Provider__: FieldGetter<__Context__, FooGetterComponent, Value = Option<String>>,
            {
                fn foo(__context__: &__Context__) -> Option<&String> {
                    __Provider__::get_field(
                            __context__,
                            ::core::marker::PhantomData::<FooGetterComponent>,
                        )
                        .as_ref()
                }
            }
            impl<__Context__, __Provider__> IsProviderFor<FooGetterComponent, __Context__, ()>
            for WithProvider<__Provider__>
            where
                __Provider__: FieldGetter<__Context__, FooGetterComponent, Value = Option<String>>,
            {}
            ")
        }
    }

    #[derive(HasField)]
    pub struct App {
        pub bar: Option<String>,
    }

    snapshot_delegate_components! {
        delegate_components! {
            App {
                FooGetterComponent: UseField<Symbol!("bar")>,
            }
        }

        expand_app(output) {
            insta::assert_snapshot!(output, @r#"
            impl DelegateComponent<FooGetterComponent> for App {
                type Delegate = UseField<Symbol!("bar")>;
            }
            impl<__Context__, __Params__> IsProviderFor<FooGetterComponent, __Context__, __Params__>
            for App
            where
                UseField<Symbol!("bar")>: IsProviderFor<FooGetterComponent, __Context__, __Params__>,
            {}
            "#)
        }
    }

    #[test]
    pub fn test_option_getter() {
        let context = App {
            bar: Some("foo".to_owned()),
        };

        assert_eq!(context.foo(), Some(&"foo".to_owned()));
    }
}

mod option_auto_getter {
    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_cgp_auto_getter;

    snapshot_cgp_auto_getter! {
        #[cgp_auto_getter]
        pub trait HasFoo {
            fn foo(&self) -> Option<&String>;
        }

        expand_has_foo(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasFoo {
                fn foo(&self) -> Option<&String>;
            }
            impl<__Context__> HasFoo for __Context__
            where
                __Context__: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value = Option<String>,
                >,
            {
                fn foo(&self) -> Option<&String> {
                    self.get_field(
                            ::core::marker::PhantomData::<
                                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                            >,
                        )
                        .as_ref()
                }
            }
            ")
        }
    }

    #[derive(HasField)]
    pub struct App {
        pub foo: Option<String>,
    }

    #[test]
    pub fn test_option_auto_getter() {
        let context = App {
            foo: Some("foo".to_owned()),
        };

        assert_eq!(context.foo(), Some(&"foo".to_owned()));
    }
}
