//! `#[derive(CgpData)]` on a *generic* record with a `where` clause.
//!
//! Every generated impl (field access, field lists, the `__Partial…` builder,
//! and its `UpdateField`/`HasField` impls) carries the struct's generic
//! parameters and forwards its `where Foo: Clone` bound, so the derive works on
//! parameterized records just as on concrete ones.
//!
//! See docs/reference/derives/derive_cgp_data.md.

use cgp_macro_test_util::snapshot_derive_cgp_data;

snapshot_derive_cgp_data! {
    #[derive(CgpData)]
    pub struct Context<Foo, Bar, Baz>
    where
        Foo: Clone,
    {
        pub foo: Foo,
        pub bar: Bar,
        pub baz: Baz,
    }

    expand_context(output) {
        insta::assert_snapshot!(output, @"
        impl<Foo, Bar, Baz> HasField<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>>
        for Context<Foo, Bar, Baz>
        where
            Foo: Clone,
        {
            type Value = Foo;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                >,
            ) -> &Self::Value {
                &self.foo
            }
        }
        impl<Foo, Bar, Baz> HasFieldMut<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>>
        for Context<Foo, Bar, Baz>
        where
            Foo: Clone,
        {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                >,
            ) -> &mut Self::Value {
                &mut self.foo
            }
        }
        impl<Foo, Bar, Baz> HasField<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>>
        for Context<Foo, Bar, Baz>
        where
            Foo: Clone,
        {
            type Value = Bar;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                >,
            ) -> &Self::Value {
                &self.bar
            }
        }
        impl<Foo, Bar, Baz> HasFieldMut<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>>
        for Context<Foo, Bar, Baz>
        where
            Foo: Clone,
        {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                >,
            ) -> &mut Self::Value {
                &mut self.bar
            }
        }
        impl<Foo, Bar, Baz> HasField<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>>
        for Context<Foo, Bar, Baz>
        where
            Foo: Clone,
        {
            type Value = Baz;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>,
                >,
            ) -> &Self::Value {
                &self.baz
            }
        }
        impl<Foo, Bar, Baz> HasFieldMut<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>>
        for Context<Foo, Bar, Baz>
        where
            Foo: Clone,
        {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>,
                >,
            ) -> &mut Self::Value {
                &mut self.baz
            }
        }
        impl<Foo, Bar, Baz> HasFields for Context<Foo, Bar, Baz>
        where
            Foo: Clone,
        {
            type Fields = Cons<
                Field<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>, Foo>,
                Cons<
                    Field<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>, Bar>,
                    Cons<Field<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>, Baz>, Nil>,
                >,
            >;
        }
        impl<Foo, Bar, Baz> HasFieldsRef for Context<Foo, Bar, Baz>
        where
            Foo: Clone,
        {
            type FieldsRef<'__a> = Cons<
                Field<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>, &'__a Foo>,
                Cons<
                    Field<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>, &'__a Bar>,
                    Cons<
                        Field<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>, &'__a Baz>,
                        Nil,
                    >,
                >,
            >
            where
                Self: '__a;
        }
        impl<Foo, Bar, Baz> FromFields for Context<Foo, Bar, Baz>
        where
            Foo: Clone,
        {
            fn from_fields(Cons(foo, Cons(bar, Cons(baz, Nil))): Self::Fields) -> Self {
                Self {
                    foo: foo.value,
                    bar: bar.value,
                    baz: baz.value,
                }
            }
        }
        impl<Foo, Bar, Baz> ToFields for Context<Foo, Bar, Baz>
        where
            Foo: Clone,
        {
            fn to_fields(self) -> Self::Fields {
                Cons(self.foo.into(), Cons(self.bar.into(), Cons(self.baz.into(), Nil)))
            }
        }
        impl<Foo, Bar, Baz> ToFieldsRef for Context<Foo, Bar, Baz>
        where
            Foo: Clone,
        {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                Cons((&self.foo).into(), Cons((&self.bar).into(), Cons((&self.baz).into(), Nil)))
            }
        }
        pub struct __PartialContext<
            Foo,
            Bar,
            Baz,
            __F0__: MapType,
            __F1__: MapType,
            __F2__: MapType,
        >
        where
            Foo: Clone,
        {
            pub foo: <__F0__ as MapType>::Map<Foo>,
            pub bar: <__F1__ as MapType>::Map<Bar>,
            pub baz: <__F2__ as MapType>::Map<Baz>,
        }
        impl<Foo, Bar, Baz> HasBuilder for Context<Foo, Bar, Baz>
        where
            Foo: Clone,
        {
            type Builder = __PartialContext<Foo, Bar, Baz, IsNothing, IsNothing, IsNothing>;
            fn builder() -> Self::Builder {
                __PartialContext {
                    foo: (),
                    bar: (),
                    baz: (),
                }
            }
        }
        impl<Foo, Bar, Baz> IntoBuilder for Context<Foo, Bar, Baz>
        where
            Foo: Clone,
        {
            type Builder = __PartialContext<Foo, Bar, Baz, IsPresent, IsPresent, IsPresent>;
            fn into_builder(self) -> Self::Builder {
                __PartialContext {
                    foo: self.foo,
                    bar: self.bar,
                    baz: self.baz,
                }
            }
        }
        impl<Foo, Bar, Baz, __F0__: MapType, __F1__: MapType, __F2__: MapType> PartialData
        for __PartialContext<Foo, Bar, Baz, __F0__, __F1__, __F2__>
        where
            Foo: Clone,
        {
            type Target = Context<Foo, Bar, Baz>;
        }
        impl<Foo, Bar, Baz> FinalizeBuild
        for __PartialContext<Foo, Bar, Baz, IsPresent, IsPresent, IsPresent>
        where
            Foo: Clone,
        {
            fn finalize_build(self) -> Self::Target {
                Context {
                    foo: self.foo,
                    bar: self.bar,
                    baz: self.baz,
                }
            }
        }
        impl<
            Foo,
            Bar,
            Baz,
            __M1__: MapType,
            __M2__: MapType,
            __F1__: MapType,
            __F2__: MapType,
        > UpdateField<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>, __M2__>
        for __PartialContext<Foo, Bar, Baz, __M1__, __F1__, __F2__>
        where
            Foo: Clone,
        {
            type Value = Foo;
            type Mapper = __M1__;
            type Output = __PartialContext<Foo, Bar, Baz, __M2__, __F1__, __F2__>;
            fn update_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                >,
                value: __M2__::Map<Self::Value>,
            ) -> (__M1__::Map<Self::Value>, Self::Output) {
                (
                    self.foo,
                    __PartialContext {
                        foo: value,
                        bar: self.bar,
                        baz: self.baz,
                    },
                )
            }
        }
        impl<
            Foo,
            Bar,
            Baz,
            __F0__: MapType,
            __M1__: MapType,
            __M2__: MapType,
            __F2__: MapType,
        > UpdateField<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>, __M2__>
        for __PartialContext<Foo, Bar, Baz, __F0__, __M1__, __F2__>
        where
            Foo: Clone,
        {
            type Value = Bar;
            type Mapper = __M1__;
            type Output = __PartialContext<Foo, Bar, Baz, __F0__, __M2__, __F2__>;
            fn update_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                >,
                value: __M2__::Map<Self::Value>,
            ) -> (__M1__::Map<Self::Value>, Self::Output) {
                (
                    self.bar,
                    __PartialContext {
                        foo: self.foo,
                        bar: value,
                        baz: self.baz,
                    },
                )
            }
        }
        impl<
            Foo,
            Bar,
            Baz,
            __F0__: MapType,
            __F1__: MapType,
            __M1__: MapType,
            __M2__: MapType,
        > UpdateField<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>, __M2__>
        for __PartialContext<Foo, Bar, Baz, __F0__, __F1__, __M1__>
        where
            Foo: Clone,
        {
            type Value = Baz;
            type Mapper = __M1__;
            type Output = __PartialContext<Foo, Bar, Baz, __F0__, __F1__, __M2__>;
            fn update_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>,
                >,
                value: __M2__::Map<Self::Value>,
            ) -> (__M1__::Map<Self::Value>, Self::Output) {
                (
                    self.baz,
                    __PartialContext {
                        foo: self.foo,
                        bar: self.bar,
                        baz: value,
                    },
                )
            }
        }
        impl<
            Foo,
            Bar,
            Baz,
            __F1__: MapType,
            __F2__: MapType,
        > HasField<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>>
        for __PartialContext<Foo, Bar, Baz, IsPresent, __F1__, __F2__>
        where
            Foo: Clone,
        {
            type Value = Foo;
            fn get_field(
                &self,
                tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                >,
            ) -> &Self::Value {
                &self.foo
            }
        }
        impl<
            Foo,
            Bar,
            Baz,
            __F0__: MapType,
            __F2__: MapType,
        > HasField<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>>
        for __PartialContext<Foo, Bar, Baz, __F0__, IsPresent, __F2__>
        where
            Foo: Clone,
        {
            type Value = Bar;
            fn get_field(
                &self,
                tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                >,
            ) -> &Self::Value {
                &self.bar
            }
        }
        impl<
            Foo,
            Bar,
            Baz,
            __F0__: MapType,
            __F1__: MapType,
        > HasField<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>>
        for __PartialContext<Foo, Bar, Baz, __F0__, __F1__, IsPresent>
        where
            Foo: Clone,
        {
            type Value = Baz;
            fn get_field(
                &self,
                tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>,
                >,
            ) -> &Self::Value {
                &self.baz
            }
        }
        ")
    }
}
