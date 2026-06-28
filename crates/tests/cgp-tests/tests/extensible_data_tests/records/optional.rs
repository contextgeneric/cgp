use cgp::extra::field::impls::{
    CanFinalizeWithDefault, FinalizeOptional, HasOptionalBuilder, SetOptional,
};
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_cgp_data;

snapshot_derive_cgp_data! {
    #[derive(CgpData)]
    pub struct Context {
        pub foo: String,
        pub bar: u64,
    }

    expand_context(output) {
        insta::assert_snapshot!(output, @"
        impl HasField<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>> for Context {
            type Value = String;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                >,
            ) -> &Self::Value {
                &self.foo
            }
        }
        impl HasFieldMut<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>> for Context {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                >,
            ) -> &mut Self::Value {
                &mut self.foo
            }
        }
        impl HasField<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>> for Context {
            type Value = u64;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                >,
            ) -> &Self::Value {
                &self.bar
            }
        }
        impl HasFieldMut<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>> for Context {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                >,
            ) -> &mut Self::Value {
                &mut self.bar
            }
        }
        impl HasFields for Context {
            type Fields = Cons<
                Field<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>, String>,
                Cons<Field<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>, u64>, Nil>,
            >;
        }
        impl HasFieldsRef for Context {
            type FieldsRef<'__a> = Cons<
                Field<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>, &'__a String>,
                Cons<Field<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>, &'__a u64>, Nil>,
            >
            where
                Self: '__a;
        }
        impl FromFields for Context {
            fn from_fields(Cons(foo, Cons(bar, Nil)): Self::Fields) -> Self {
                Self {
                    foo: foo.value,
                    bar: bar.value,
                }
            }
        }
        impl ToFields for Context {
            fn to_fields(self) -> Self::Fields {
                Cons(self.foo.into(), Cons(self.bar.into(), Nil))
            }
        }
        impl ToFieldsRef for Context {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                Cons((&self.foo).into(), Cons((&self.bar).into(), Nil))
            }
        }
        pub struct __PartialContext<__F0__: MapType, __F1__: MapType> {
            pub foo: <__F0__ as MapType>::Map<String>,
            pub bar: <__F1__ as MapType>::Map<u64>,
        }
        impl HasBuilder for Context {
            type Builder = __PartialContext<IsNothing, IsNothing>;
            fn builder() -> Self::Builder {
                __PartialContext {
                    foo: (),
                    bar: (),
                }
            }
        }
        impl IntoBuilder for Context {
            type Builder = __PartialContext<IsPresent, IsPresent>;
            fn into_builder(self) -> Self::Builder {
                __PartialContext {
                    foo: self.foo,
                    bar: self.bar,
                }
            }
        }
        impl<__F0__: MapType, __F1__: MapType> PartialData for __PartialContext<__F0__, __F1__> {
            type Target = Context;
        }
        impl FinalizeBuild for __PartialContext<IsPresent, IsPresent> {
            fn finalize_build(self) -> Self::Target {
                Context {
                    foo: self.foo,
                    bar: self.bar,
                }
            }
        }
        impl<
            __M1__: MapType,
            __M2__: MapType,
            __F1__: MapType,
        > UpdateField<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>, __M2__>
        for __PartialContext<__M1__, __F1__> {
            type Value = String;
            type Mapper = __M1__;
            type Output = __PartialContext<__M2__, __F1__>;
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
                    },
                )
            }
        }
        impl<
            __F0__: MapType,
            __M1__: MapType,
            __M2__: MapType,
        > UpdateField<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>, __M2__>
        for __PartialContext<__F0__, __M1__> {
            type Value = u64;
            type Mapper = __M1__;
            type Output = __PartialContext<__F0__, __M2__>;
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
                    },
                )
            }
        }
        impl<__F1__: MapType> HasField<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>>
        for __PartialContext<IsPresent, __F1__> {
            type Value = String;
            fn get_field(
                &self,
                tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                >,
            ) -> &Self::Value {
                &self.foo
            }
        }
        impl<__F0__: MapType> HasField<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>>
        for __PartialContext<__F0__, IsPresent> {
            type Value = u64;
            fn get_field(
                &self,
                tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                >,
            ) -> &Self::Value {
                &self.bar
            }
        }
        ")
    }
}

#[test]
fn test_optional_fields() {
    let builder = Context::optional_builder();

    let builder = builder
        .set(PhantomData::<Symbol!("foo")>, "foo".to_owned())
        .set(PhantomData::<Symbol!("bar")>, 42);

    let (replaced, builder) = builder.set_optional(PhantomData::<Symbol!("foo")>, "bar".to_owned());
    assert_eq!(replaced, Some("foo".to_owned()));

    let context = builder.finalize_optional().unwrap();
    assert_eq!(context.foo, "bar");
    assert_eq!(context.bar, 42);
}

#[test]
fn test_optional_and_default_fields() {
    let builder = Context::optional_builder();

    let builder = builder.set(PhantomData::<Symbol!("foo")>, "foo".to_owned());

    let context = builder.finalize_with_default();

    assert_eq!(context.foo, "foo");
    assert_eq!(context.bar, 0);
}
