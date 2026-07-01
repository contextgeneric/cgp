//! Canonical expansion of `#[derive(CgpData)]` for a record (named-field struct).
//!
//! `#[derive(CgpData)]` derives the whole extensible-record spine at once: a
//! `HasField`/`HasFieldMut` impl per field, `HasFields`/`HasFieldsRef` exposing
//! the type-level field list, the `FromFields`/`ToFields`/`ToFieldsRef`
//! conversions, and a generated `__Partial…` builder type that powers the
//! extensible builder pattern (`HasBuilder`/`IntoBuilder`, `FinalizeBuild`,
//! `UpdateField`). This is the reference snapshot for that expansion; other
//! record tests reuse `#[derive(CgpData)]` without re-snapshotting it.
//!
//! The runtime test exercises the builder: start from `builder()`, set each
//! field with `build_field`, then `finalize_build` once every field is present.
//!
//! See docs/reference/derives/derive_cgp_data.md and
//! docs/reference/traits/has_builder.md.

use core::marker::PhantomData;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_cgp_data;

snapshot_derive_cgp_data! {
    #[derive(CgpData)]
    #[derive(Debug, Eq, PartialEq)]
    pub struct FooBarBaz {
        pub foo: u64,
        pub bar: String,
        pub baz: bool,
    }

    expand_foo_bar_baz(output) {
            insta::assert_snapshot!(output, @"
            impl HasField<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>> for FooBarBaz {
                type Value = u64;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<
                        Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    >,
                ) -> &Self::Value {
                    &self.foo
                }
            }
            impl HasFieldMut<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>> for FooBarBaz {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<
                        Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    >,
                ) -> &mut Self::Value {
                    &mut self.foo
                }
            }
            impl HasField<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>> for FooBarBaz {
                type Value = String;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<
                        Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                    >,
                ) -> &Self::Value {
                    &self.bar
                }
            }
            impl HasFieldMut<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>> for FooBarBaz {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<
                        Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                    >,
                ) -> &mut Self::Value {
                    &mut self.bar
                }
            }
            impl HasField<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>> for FooBarBaz {
                type Value = bool;
                fn get_field(
                    &self,
                    key: ::core::marker::PhantomData<
                        Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>,
                    >,
                ) -> &Self::Value {
                    &self.baz
                }
            }
            impl HasFieldMut<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>> for FooBarBaz {
                fn get_field_mut(
                    &mut self,
                    key: ::core::marker::PhantomData<
                        Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>,
                    >,
                ) -> &mut Self::Value {
                    &mut self.baz
                }
            }
            impl HasFields for FooBarBaz {
                type Fields = Cons<
                    Field<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>, u64>,
                    Cons<
                        Field<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>, String>,
                        Cons<Field<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>, bool>, Nil>,
                    >,
                >;
            }
            impl HasFieldsRef for FooBarBaz {
                type FieldsRef<'__a> = Cons<
                    Field<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>, &'__a u64>,
                    Cons<
                        Field<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>, &'__a String>,
                        Cons<
                            Field<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>, &'__a bool>,
                            Nil,
                        >,
                    >,
                >
                where
                    Self: '__a;
            }
            impl FromFields for FooBarBaz {
                fn from_fields(Cons(foo, Cons(bar, Cons(baz, Nil))): Self::Fields) -> Self {
                    Self {
                        foo: foo.value,
                        bar: bar.value,
                        baz: baz.value,
                    }
                }
            }
            impl ToFields for FooBarBaz {
                fn to_fields(self) -> Self::Fields {
                    Cons(self.foo.into(), Cons(self.bar.into(), Cons(self.baz.into(), Nil)))
                }
            }
            impl ToFieldsRef for FooBarBaz {
                fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
                where
                    Self: '__a,
                {
                    Cons((&self.foo).into(), Cons((&self.bar).into(), Cons((&self.baz).into(), Nil)))
                }
            }
            pub struct __PartialFooBarBaz<__F0__: MapType, __F1__: MapType, __F2__: MapType> {
                pub foo: <__F0__ as MapType>::Map<u64>,
                pub bar: <__F1__ as MapType>::Map<String>,
                pub baz: <__F2__ as MapType>::Map<bool>,
            }
            impl HasBuilder for FooBarBaz {
                type Builder = __PartialFooBarBaz<IsNothing, IsNothing, IsNothing>;
                fn builder() -> Self::Builder {
                    __PartialFooBarBaz {
                        foo: (),
                        bar: (),
                        baz: (),
                    }
                }
            }
            impl IntoBuilder for FooBarBaz {
                type Builder = __PartialFooBarBaz<IsPresent, IsPresent, IsPresent>;
                fn into_builder(self) -> Self::Builder {
                    __PartialFooBarBaz {
                        foo: self.foo,
                        bar: self.bar,
                        baz: self.baz,
                    }
                }
            }
            impl<__F0__: MapType, __F1__: MapType, __F2__: MapType> PartialData
            for __PartialFooBarBaz<__F0__, __F1__, __F2__> {
                type Target = FooBarBaz;
            }
            impl FinalizeBuild for __PartialFooBarBaz<IsPresent, IsPresent, IsPresent> {
                fn finalize_build(self) -> Self::Target {
                    FooBarBaz {
                        foo: self.foo,
                        bar: self.bar,
                        baz: self.baz,
                    }
                }
            }
            impl<
                __M1__: MapType,
                __M2__: MapType,
                __F1__: MapType,
                __F2__: MapType,
            > UpdateField<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>, __M2__>
            for __PartialFooBarBaz<__M1__, __F1__, __F2__> {
                type Value = u64;
                type Mapper = __M1__;
                type Output = __PartialFooBarBaz<__M2__, __F1__, __F2__>;
                fn update_field(
                    self,
                    _tag: ::core::marker::PhantomData<
                        Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    >,
                    value: __M2__::Map<Self::Value>,
                ) -> (__M1__::Map<Self::Value>, Self::Output) {
                    (
                        self.foo,
                        __PartialFooBarBaz {
                            foo: value,
                            bar: self.bar,
                            baz: self.baz,
                        },
                    )
                }
            }
            impl<
                __F0__: MapType,
                __M1__: MapType,
                __M2__: MapType,
                __F2__: MapType,
            > UpdateField<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>, __M2__>
            for __PartialFooBarBaz<__F0__, __M1__, __F2__> {
                type Value = String;
                type Mapper = __M1__;
                type Output = __PartialFooBarBaz<__F0__, __M2__, __F2__>;
                fn update_field(
                    self,
                    _tag: ::core::marker::PhantomData<
                        Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                    >,
                    value: __M2__::Map<Self::Value>,
                ) -> (__M1__::Map<Self::Value>, Self::Output) {
                    (
                        self.bar,
                        __PartialFooBarBaz {
                            foo: self.foo,
                            bar: value,
                            baz: self.baz,
                        },
                    )
                }
            }
            impl<
                __F0__: MapType,
                __F1__: MapType,
                __M1__: MapType,
                __M2__: MapType,
            > UpdateField<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>, __M2__>
            for __PartialFooBarBaz<__F0__, __F1__, __M1__> {
                type Value = bool;
                type Mapper = __M1__;
                type Output = __PartialFooBarBaz<__F0__, __F1__, __M2__>;
                fn update_field(
                    self,
                    _tag: ::core::marker::PhantomData<
                        Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>,
                    >,
                    value: __M2__::Map<Self::Value>,
                ) -> (__M1__::Map<Self::Value>, Self::Output) {
                    (
                        self.baz,
                        __PartialFooBarBaz {
                            foo: self.foo,
                            bar: self.bar,
                            baz: value,
                        },
                    )
                }
            }
            impl<
                __F1__: MapType,
                __F2__: MapType,
            > HasField<Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>>
            for __PartialFooBarBaz<IsPresent, __F1__, __F2__> {
                type Value = u64;
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
                __F0__: MapType,
                __F2__: MapType,
            > HasField<Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>>
            for __PartialFooBarBaz<__F0__, IsPresent, __F2__> {
                type Value = String;
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
                __F0__: MapType,
                __F1__: MapType,
            > HasField<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>>
            for __PartialFooBarBaz<__F0__, __F1__, IsPresent> {
                type Value = bool;
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

#[test]
fn test_basic_builder() {
    let context: FooBarBaz = FooBarBaz::builder()
        .build_field(PhantomData::<Symbol!("foo")>, 1)
        .build_field(PhantomData::<Symbol!("bar")>, "bar".to_owned())
        .build_field(PhantomData::<Symbol!("baz")>, true)
        .finalize_build();

    assert_eq!(context.foo, 1);
    assert_eq!(context.bar, "bar");
    assert!(context.baz);
}
