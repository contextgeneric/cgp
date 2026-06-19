mod mref_getter {
    use cgp::core::field::types::MRef;
    use cgp::prelude::*;
    use cgp_macro_test_util::{snapshot_cgp_getter, snapshot_delegate_components};

    snapshot_cgp_getter! {
        #[cgp_getter]
        pub trait HasFoo {
            fn foo(&self) -> MRef<'_, String>;
        }

        expand_has_foo(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasFoo {
                fn foo(&self) -> MRef<'_, String>;
            }
            impl<__Context__> HasFoo for __Context__
            where
                __Context__: FooGetter<__Context__>,
            {
                fn foo(&self) -> MRef<'_, String> {
                    __Context__::foo(self)
                }
            }
            pub trait FooGetter<__Context__>: IsProviderFor<FooGetterComponent, __Context__, ()> {
                fn foo(__context__: &__Context__) -> MRef<'_, String>;
            }
            impl<__Provider__, __Context__> FooGetter<__Context__> for __Provider__
            where
                __Provider__: DelegateComponent<FooGetterComponent>
                    + IsProviderFor<FooGetterComponent, __Context__, ()>,
                <__Provider__ as DelegateComponent<
                    FooGetterComponent,
                >>::Delegate: FooGetter<__Context__>,
            {
                fn foo(__context__: &__Context__) -> MRef<'_, String> {
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
                fn foo(__context__: &__Context__) -> MRef<'_, String> {
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
                fn foo(__context__: &__Context__) -> MRef<'_, String> {
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
                    Value = String,
                >,
            {
                fn foo(__context__: &__Context__) -> MRef<'_, String> {
                    MRef::Ref(
                        __context__
                            .get_field(
                                ::core::marker::PhantomData::<
                                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                                >,
                            ),
                    )
                }
            }
            impl<__Context__> IsProviderFor<FooGetterComponent, __Context__, ()> for UseFields
            where
                __Context__: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value = String,
                >,
            {}
            impl<__Context__, __Tag__> FooGetter<__Context__> for UseField<__Tag__>
            where
                __Context__: HasField<__Tag__, Value = String>,
            {
                fn foo(__context__: &__Context__) -> MRef<'_, String> {
                    MRef::Ref(__context__.get_field(::core::marker::PhantomData::<__Tag__>))
                }
            }
            impl<__Context__, __Tag__> IsProviderFor<FooGetterComponent, __Context__, ()>
            for UseField<__Tag__>
            where
                __Context__: HasField<__Tag__, Value = String>,
            {}
            impl<__Context__, __Provider__> FooGetter<__Context__> for WithProvider<__Provider__>
            where
                __Provider__: FieldGetter<__Context__, FooGetterComponent, Value = String>,
            {
                fn foo(__context__: &__Context__) -> MRef<'_, String> {
                    MRef::Ref(
                        __Provider__::get_field(
                            __context__,
                            ::core::marker::PhantomData::<FooGetterComponent>,
                        ),
                    )
                }
            }
            impl<__Context__, __Provider__> IsProviderFor<FooGetterComponent, __Context__, ()>
            for WithProvider<__Provider__>
            where
                __Provider__: FieldGetter<__Context__, FooGetterComponent, Value = String>,
            {}
            ")
        }
    }

    #[derive(HasField)]
    pub struct App {
        pub bar: String,
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
    pub fn test_mref_getter() {
        let context = App { bar: "foo".into() };

        assert_eq!(context.foo().as_ref(), "foo");
    }
}

mod mref_auto_getter {
    use cgp::core::field::types::MRef;
    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_cgp_auto_getter;

    snapshot_cgp_auto_getter! {
        #[cgp_auto_getter]
        pub trait HasFoo {
            fn foo(&self) -> MRef<'_, String>;
        }

        expand_has_foo(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasFoo {
                fn foo(&self) -> MRef<'_, String>;
            }
            impl<__Context__> HasFoo for __Context__
            where
                __Context__: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value = String,
                >,
            {
                fn foo(&self) -> MRef<'_, String> {
                    MRef::Ref(
                        self
                            .get_field(
                                ::core::marker::PhantomData::<
                                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                                >,
                            ),
                    )
                }
            }
            ")
        }
    }

    #[derive(HasField)]
    pub struct App {
        pub foo: String,
    }

    #[test]
    pub fn test_mref_auto_getter() {
        let context = App { foo: "foo".into() };

        assert_eq!(context.foo().as_ref(), "foo");
    }
}
