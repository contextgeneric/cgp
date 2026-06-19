mod slice_getter {
    use cgp::prelude::*;
    use cgp_macro_test_util::{snapshot_cgp_getter, snapshot_delegate_components};

    snapshot_cgp_getter! {
        #[cgp_getter]
        pub trait HasFoo {
            fn foo(&self) -> &[u8];
        }

        expand_has_foo(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasFoo {
                fn foo(&self) -> &[u8];
            }
            impl<__Context__> HasFoo for __Context__
            where
                __Context__: FooGetter<__Context__>,
            {
                fn foo(&self) -> &[u8] {
                    __Context__::foo(self)
                }
            }
            pub trait FooGetter<__Context__>: IsProviderFor<FooGetterComponent, __Context__, ()> {
                fn foo(__context__: &__Context__) -> &[u8];
            }
            impl<__Provider__, __Context__> FooGetter<__Context__> for __Provider__
            where
                __Provider__: DelegateComponent<FooGetterComponent>
                    + IsProviderFor<FooGetterComponent, __Context__, ()>,
                <__Provider__ as DelegateComponent<
                    FooGetterComponent,
                >>::Delegate: FooGetter<__Context__>,
            {
                fn foo(__context__: &__Context__) -> &[u8] {
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
                fn foo(__context__: &__Context__) -> &[u8] {
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
                fn foo(__context__: &__Context__) -> &[u8] {
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
                    Value: AsRef<[u8]> + 'static,
                >,
            {
                fn foo(__context__: &__Context__) -> &[u8] {
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
                    Value: AsRef<[u8]> + 'static,
                >,
            {}
            impl<__Context__, __Tag__> FooGetter<__Context__> for UseField<__Tag__>
            where
                __Context__: HasField<__Tag__, Value: AsRef<[u8]> + 'static>,
            {
                fn foo(__context__: &__Context__) -> &[u8] {
                    __context__.get_field(::core::marker::PhantomData::<__Tag__>).as_ref()
                }
            }
            impl<__Context__, __Tag__> IsProviderFor<FooGetterComponent, __Context__, ()>
            for UseField<__Tag__>
            where
                __Context__: HasField<__Tag__, Value: AsRef<[u8]> + 'static>,
            {}
            impl<__Context__, __Provider__> FooGetter<__Context__> for WithProvider<__Provider__>
            where
                __Provider__: FieldGetter<
                    __Context__,
                    FooGetterComponent,
                    Value: AsRef<[u8]> + 'static,
                >,
            {
                fn foo(__context__: &__Context__) -> &[u8] {
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
                __Provider__: FieldGetter<
                    __Context__,
                    FooGetterComponent,
                    Value: AsRef<[u8]> + 'static,
                >,
            {}
            ")
        }
    }

    #[derive(HasField)]
    pub struct App {
        pub bar: Vec<u8>,
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
    pub fn test_slice_getter() {
        let context = App { bar: vec![1, 2, 3] };

        assert_eq!(context.foo(), &[1, 2, 3]);
    }
}

mod slice_auto_getter {
    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_cgp_auto_getter;

    snapshot_cgp_auto_getter! {
        #[cgp_auto_getter]
        pub trait HasFoo {
            fn foo(&self) -> &[u8];
        }

        expand_has_foo(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasFoo {
                fn foo(&self) -> &[u8];
            }
            impl<__Context__> HasFoo for __Context__
            where
                __Context__: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value: AsRef<[u8]> + 'static,
                >,
            {
                fn foo(&self) -> &[u8] {
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
        pub foo: Vec<u8>,
    }

    #[test]
    pub fn test_slice_auto_getter() {
        let context = App { foo: vec![1, 2, 3] };

        assert_eq!(context.foo(), &[1, 2, 3]);
    }
}
