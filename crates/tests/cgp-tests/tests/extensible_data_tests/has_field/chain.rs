pub mod chained_getter {
    use core::marker::PhantomData;

    use cgp::core::field::impls::ChainGetters;
    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_derive_has_field;

    snapshot_derive_has_field! {
        #[derive(HasField)]
        pub struct Inner {
            pub name: String,
        }

        expand_inner(output) {
            insta::assert_snapshot!(output, @"
            impl HasField<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>> for Inner {
                type Value = String;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<
                        Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    >,
                ) -> &Self::Value {
                    &self.name
                }
            }
            impl HasFieldMut<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>>
            for Inner {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<
                        Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    >,
                ) -> &mut Self::Value {
                    &mut self.name
                }
            }
            ")
        }
    }

    snapshot_derive_has_field! {
        #[derive(HasField)]
        pub struct Outer {
            pub inner: Inner,
        }

        expand_outer(output) {
            insta::assert_snapshot!(output, @"
            impl HasField<Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>>
            for Outer {
                type Value = Inner;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<
                        Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>,
                    >,
                ) -> &Self::Value {
                    &self.inner
                }
            }
            impl HasFieldMut<
                Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>,
            > for Outer {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<
                        Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>,
                    >,
                ) -> &mut Self::Value {
                    &mut self.inner
                }
            }
            ")
        }
    }

    #[test]
    fn test_chained_getter() {
        let context = Outer {
            inner: Inner {
                name: "test".to_owned(),
            },
        };

        let name: &String = <ChainGetters<
            Product![UseField<Symbol!("inner")>, UseField<Symbol!("name")>],
        >>::get_field(&context, PhantomData::<()>);
        assert_eq!(name, "test");
    }
}

mod chained_getter_with_outer_life {
    use core::marker::PhantomData;

    use cgp::core::field::impls::ChainGetters;
    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_derive_has_field;

    snapshot_derive_has_field! {
        #[derive(HasField)]
        pub struct Outer<'a> {
            pub inner: &'a Inner,
        }

        expand_outer(output) {
            insta::assert_snapshot!(output, @"
            impl<
                'a,
            > HasField<Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>>
            for Outer<'a> {
                type Value = &'a Inner;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<
                        Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>,
                    >,
                ) -> &Self::Value {
                    &self.inner
                }
            }
            impl<
                'a,
            > HasFieldMut<Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>>
            for Outer<'a> {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<
                        Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>,
                    >,
                ) -> &mut Self::Value {
                    &mut self.inner
                }
            }
            ")
        }
    }

    snapshot_derive_has_field! {
        #[derive(HasField)]
        pub struct Inner {
            pub name: String,
        }

        expand_inner(output) {
            insta::assert_snapshot!(output, @"
            impl HasField<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>> for Inner {
                type Value = String;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<
                        Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    >,
                ) -> &Self::Value {
                    &self.name
                }
            }
            impl HasFieldMut<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>>
            for Inner {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<
                        Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    >,
                ) -> &mut Self::Value {
                    &mut self.name
                }
            }
            ")
        }
    }

    #[test]
    fn test_chained_getter_with_outer_life() {
        let context = Outer {
            inner: &Inner {
                name: "test".to_owned(),
            },
        };

        let name: &String = <ChainGetters<
            Product![UseField<Symbol!("inner")>, UseField<Symbol!("name")>],
        >>::get_field(&context, PhantomData::<()>);
        assert_eq!(name, "test");
    }
}

mod chained_getter_with_inner_life {
    use core::marker::PhantomData;

    use cgp::core::field::impls::ChainGetters;
    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_derive_has_field;

    snapshot_derive_has_field! {
        #[derive(HasField)]
        pub struct Outer<'a> {
            pub inner: Inner<'a>,
        }

        expand_outer(output) {
            insta::assert_snapshot!(output, @"
            impl<
                'a,
            > HasField<Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>>
            for Outer<'a> {
                type Value = Inner<'a>;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<
                        Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>,
                    >,
                ) -> &Self::Value {
                    &self.inner
                }
            }
            impl<
                'a,
            > HasFieldMut<Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>>
            for Outer<'a> {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<
                        Symbol<5, Chars<'i', Chars<'n', Chars<'n', Chars<'e', Chars<'r', Nil>>>>>>,
                    >,
                ) -> &mut Self::Value {
                    &mut self.inner
                }
            }
            ")
        }
    }

    snapshot_derive_has_field! {
        #[derive(HasField)]
        pub struct Inner<'a> {
            pub name: &'a String,
        }

        expand_inner(output) {
            insta::assert_snapshot!(output, @"
            impl<'a> HasField<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>>
            for Inner<'a> {
                type Value = &'a String;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<
                        Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    >,
                ) -> &Self::Value {
                    &self.name
                }
            }
            impl<'a> HasFieldMut<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>>
            for Inner<'a> {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<
                        Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    >,
                ) -> &mut Self::Value {
                    &mut self.name
                }
            }
            ")
        }
    }

    #[test]
    fn test_chained_getter_with_inner_life() {
        let context = Outer {
            inner: Inner {
                name: &"test".to_owned(),
            },
        };

        let name: &String = <ChainGetters<
            Product![UseField<Symbol!("inner")>, UseField<Symbol!("name")>],
        >>::get_field(&context, PhantomData::<()>);

        assert_eq!(name, "test");
    }
}

mod deeply_nested_getter {
    use cgp::core::field::impls::ChainGetters;
    use cgp::prelude::*;
    use cgp_macro_test_util::{
        snapshot_cgp_getter, snapshot_delegate_and_check_components, snapshot_derive_has_field,
    };

    snapshot_derive_has_field! {
        #[derive(HasField)]
        pub struct A {
            pub b: B,
        }

        expand_a(output) {
            insta::assert_snapshot!(output, @"
            impl HasField<Symbol<1, Chars<'b', Nil>>> for A {
                type Value = B;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<Symbol<1, Chars<'b', Nil>>>,
                ) -> &Self::Value {
                    &self.b
                }
            }
            impl HasFieldMut<Symbol<1, Chars<'b', Nil>>> for A {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<Symbol<1, Chars<'b', Nil>>>,
                ) -> &mut Self::Value {
                    &mut self.b
                }
            }
            ")
        }
    }

    snapshot_derive_has_field! {
        #[derive(HasField)]
        pub struct B {
            pub c: C,
        }

        expand_b(output) {
            insta::assert_snapshot!(output, @"
            impl HasField<Symbol<1, Chars<'c', Nil>>> for B {
                type Value = C;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<Symbol<1, Chars<'c', Nil>>>,
                ) -> &Self::Value {
                    &self.c
                }
            }
            impl HasFieldMut<Symbol<1, Chars<'c', Nil>>> for B {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<Symbol<1, Chars<'c', Nil>>>,
                ) -> &mut Self::Value {
                    &mut self.c
                }
            }
            ")
        }
    }

    snapshot_derive_has_field! {
        #[derive(HasField)]
        pub struct C {
            pub d: D,
        }

        expand_c(output) {
            insta::assert_snapshot!(output, @"
            impl HasField<Symbol<1, Chars<'d', Nil>>> for C {
                type Value = D;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<Symbol<1, Chars<'d', Nil>>>,
                ) -> &Self::Value {
                    &self.d
                }
            }
            impl HasFieldMut<Symbol<1, Chars<'d', Nil>>> for C {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<Symbol<1, Chars<'d', Nil>>>,
                ) -> &mut Self::Value {
                    &mut self.d
                }
            }
            ")
        }
    }

    snapshot_derive_has_field! {
        #[derive(HasField)]
        pub struct D {
            pub name: String,
        }

        expand_d(output) {
            insta::assert_snapshot!(output, @"
            impl HasField<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>> for D {
                type Value = String;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<
                        Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    >,
                ) -> &Self::Value {
                    &self.name
                }
            }
            impl HasFieldMut<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>> for D {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<
                        Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    >,
                ) -> &mut Self::Value {
                    &mut self.name
                }
            }
            ")
        }
    }

    snapshot_derive_has_field! {
        #[derive(HasField)]
        pub struct MyContext {
            pub a: A,
        }

        expand_my_context_struct(output) {
            insta::assert_snapshot!(output, @"
            impl HasField<Symbol<1, Chars<'a', Nil>>> for MyContext {
                type Value = A;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<Symbol<1, Chars<'a', Nil>>>,
                ) -> &Self::Value {
                    &self.a
                }
            }
            impl HasFieldMut<Symbol<1, Chars<'a', Nil>>> for MyContext {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<Symbol<1, Chars<'a', Nil>>>,
                ) -> &mut Self::Value {
                    &mut self.a
                }
            }
            ")
        }
    }

    snapshot_cgp_getter! {
        #[cgp_getter]
        pub trait HasName {
            fn name(&self) -> &str;
        }

        expand_has_name(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasName {
                fn name(&self) -> &str;
            }
            impl<__Context__> HasName for __Context__
            where
                __Context__: NameGetter<__Context__>,
            {
                fn name(&self) -> &str {
                    __Context__::name(self)
                }
            }
            pub trait NameGetter<__Context__>: IsProviderFor<NameGetterComponent, __Context__, ()> {
                fn name(__context__: &__Context__) -> &str;
            }
            impl<__Provider__, __Context__> NameGetter<__Context__> for __Provider__
            where
                __Provider__: DelegateComponent<NameGetterComponent>
                    + IsProviderFor<NameGetterComponent, __Context__, ()>,
                <__Provider__ as DelegateComponent<
                    NameGetterComponent,
                >>::Delegate: NameGetter<__Context__>,
            {
                fn name(__context__: &__Context__) -> &str {
                    <__Provider__ as DelegateComponent<
                        NameGetterComponent,
                    >>::Delegate::name(__context__)
                }
            }
            pub struct NameGetterComponent;
            impl<__Context__> NameGetter<__Context__> for UseContext
            where
                __Context__: HasName,
            {
                fn name(__context__: &__Context__) -> &str {
                    __Context__::name(__context__)
                }
            }
            impl<__Context__> IsProviderFor<NameGetterComponent, __Context__, ()> for UseContext
            where
                __Context__: HasName,
            {}
            impl<__Context__, __Components__, __Path__> NameGetter<__Context__>
            for RedirectLookup<__Components__, __Path__>
            where
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<__Path__>>::Delegate: NameGetter<__Context__>,
            {
                fn name(__context__: &__Context__) -> &str {
                    <__Components__ as DelegateComponent<__Path__>>::Delegate::name(__context__)
                }
            }
            impl<
                __Context__,
                __Components__,
                __Path__,
            > IsProviderFor<NameGetterComponent, __Context__, ()>
            for RedirectLookup<__Components__, __Path__>
            where
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate: IsProviderFor<NameGetterComponent, __Context__, ()>
                    + NameGetter<__Context__>,
            {}
            impl<__Context__> NameGetter<__Context__> for UseFields
            where
                __Context__: HasField<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    Value = String,
                >,
            {
                fn name(__context__: &__Context__) -> &str {
                    __context__
                        .get_field(
                            ::core::marker::PhantomData::<
                                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                            >,
                        )
                        .as_str()
                }
            }
            impl<__Context__> IsProviderFor<NameGetterComponent, __Context__, ()> for UseFields
            where
                __Context__: HasField<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    Value = String,
                >,
            {}
            impl<__Context__, __Tag__> NameGetter<__Context__> for UseField<__Tag__>
            where
                __Context__: HasField<__Tag__, Value = String>,
            {
                fn name(__context__: &__Context__) -> &str {
                    __context__.get_field(::core::marker::PhantomData::<__Tag__>).as_str()
                }
            }
            impl<__Context__, __Tag__> IsProviderFor<NameGetterComponent, __Context__, ()>
            for UseField<__Tag__>
            where
                __Context__: HasField<__Tag__, Value = String>,
            {}
            impl<__Context__, __Provider__> NameGetter<__Context__> for WithProvider<__Provider__>
            where
                __Provider__: FieldGetter<__Context__, NameGetterComponent, Value = String>,
            {
                fn name(__context__: &__Context__) -> &str {
                    __Provider__::get_field(
                            __context__,
                            ::core::marker::PhantomData::<NameGetterComponent>,
                        )
                        .as_str()
                }
            }
            impl<__Context__, __Provider__> IsProviderFor<NameGetterComponent, __Context__, ()>
            for WithProvider<__Provider__>
            where
                __Provider__: FieldGetter<__Context__, NameGetterComponent, Value = String>,
            {}
            ")
        }
    }

    snapshot_delegate_and_check_components! {
        delegate_and_check_components! {
            MyContext {
                NameGetterComponent: WithProvider<
                    ChainGetters<Product![
                        UseField<Symbol!("a")>,
                        UseField<Symbol!("b")>,
                        UseField<Symbol!("c")>,
                        UseField<Symbol!("d")>,
                        UseField<Symbol!("name")>
                    ]>>
            }
        }

        expand_my_context(output) {
            insta::assert_snapshot!(output, @r#"
            impl DelegateComponent<NameGetterComponent> for MyContext {
                type Delegate = WithProvider<
                    ChainGetters<
                        Product![
                            UseField < Symbol!("a") >, UseField < Symbol!("b") >, UseField <
                            Symbol!("c") >, UseField < Symbol!("d") >, UseField < Symbol!("name") >
                        ],
                    >,
                >;
            }
            impl<__Context__, __Params__> IsProviderFor<NameGetterComponent, __Context__, __Params__>
            for MyContext
            where
                WithProvider<
                    ChainGetters<
                        Product![
                            UseField < Symbol!("a") >, UseField < Symbol!("b") >, UseField <
                            Symbol!("c") >, UseField < Symbol!("d") >, UseField < Symbol!("name") >
                        ],
                    >,
                >: IsProviderFor<NameGetterComponent, __Context__, __Params__>,
            {}
            trait __CanUseMyContext<
                __Component__,
                __Params__: ?Sized,
            >: CanUseComponent<__Component__, __Params__> {}
            impl __CanUseMyContext<NameGetterComponent, ()> for MyContext {}
            "#)
        }
    }

    #[test]
    fn test_deeply_nested_getter() {
        let context = MyContext {
            a: A {
                b: B {
                    c: C {
                        d: D {
                            name: "test".to_owned(),
                        },
                    },
                },
            },
        };

        assert_eq!(context.name(), "test");
    }
}
