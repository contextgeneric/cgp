//! `#[derive(CgpData)]` on a *tuple* (unnamed-field) struct.
//!
//! Positional fields are tagged with `Index<N>` instead of `Symbol!`, so the
//! whole record spine — field access, the `Cons` field list, the `__Partial…`
//! builder — is keyed by index. Otherwise the expansion mirrors the named-field
//! case pinned by `record_derive`.
//!
//! See docs/reference/derives/derive_cgp_data.md.

use cgp_macro_test_util::snapshot_derive_cgp_data;

snapshot_derive_cgp_data! {
    #[derive(CgpData)]
    pub struct Context(pub u64, pub String, pub bool);

    expand_context(output) {
        insta::assert_snapshot!(output, @"
        impl HasField<Index<0>> for Context {
            type Value = u64;
            fn get_field(&self, key: ::core::marker::PhantomData<Index<0>>) -> &Self::Value {
                &self.0
            }
        }
        impl HasFieldMut<Index<0>> for Context {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Index<0>>,
            ) -> &mut Self::Value {
                &mut self.0
            }
        }
        impl HasField<Index<1>> for Context {
            type Value = String;
            fn get_field(&self, key: ::core::marker::PhantomData<Index<1>>) -> &Self::Value {
                &self.1
            }
        }
        impl HasFieldMut<Index<1>> for Context {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Index<1>>,
            ) -> &mut Self::Value {
                &mut self.1
            }
        }
        impl HasField<Index<2>> for Context {
            type Value = bool;
            fn get_field(&self, key: ::core::marker::PhantomData<Index<2>>) -> &Self::Value {
                &self.2
            }
        }
        impl HasFieldMut<Index<2>> for Context {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Index<2>>,
            ) -> &mut Self::Value {
                &mut self.2
            }
        }
        impl HasFields for Context {
            type Fields = Cons<
                Field<Index<0>, u64>,
                Cons<Field<Index<1>, String>, Cons<Field<Index<2>, bool>, Nil>>,
            >;
        }
        impl HasFieldsRef for Context {
            type FieldsRef<'__a> = Cons<
                Field<Index<0>, &'__a u64>,
                Cons<Field<Index<1>, &'__a String>, Cons<Field<Index<2>, &'__a bool>, Nil>>,
            >
            where
                Self: '__a;
        }
        impl FromFields for Context {
            fn from_fields(
                Cons(field_2, Cons(field_1, Cons(field_0, Nil))): Self::Fields,
            ) -> Self {
                Self(field_2.value, field_1.value, field_0.value)
            }
        }
        impl ToFields for Context {
            fn to_fields(self) -> Self::Fields {
                Cons(self.0.into(), Cons(self.1.into(), Cons(self.2.into(), Nil)))
            }
        }
        impl ToFieldsRef for Context {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                Cons((&self.0).into(), Cons((&self.1).into(), Cons((&self.2).into(), Nil)))
            }
        }
        pub struct __PartialContext<__F0__: MapType, __F1__: MapType, __F2__: MapType>(
            pub <__F0__ as MapType>::Map<u64>,
            pub <__F1__ as MapType>::Map<String>,
            pub <__F2__ as MapType>::Map<bool>,
        );
        impl HasBuilder for Context {
            type Builder = __PartialContext<IsNothing, IsNothing, IsNothing>;
            fn builder() -> Self::Builder {
                __PartialContext {
                    0: (),
                    1: (),
                    2: (),
                }
            }
        }
        impl IntoBuilder for Context {
            type Builder = __PartialContext<IsPresent, IsPresent, IsPresent>;
            fn into_builder(self) -> Self::Builder {
                __PartialContext {
                    0: self.0,
                    1: self.1,
                    2: self.2,
                }
            }
        }
        impl<__F0__: MapType, __F1__: MapType, __F2__: MapType> PartialData
        for __PartialContext<__F0__, __F1__, __F2__> {
            type Target = Context;
        }
        impl FinalizeBuild for __PartialContext<IsPresent, IsPresent, IsPresent> {
            fn finalize_build(self) -> Self::Target {
                Context {
                    0: self.0,
                    1: self.1,
                    2: self.2,
                }
            }
        }
        impl<
            __M1__: MapType,
            __M2__: MapType,
            __F1__: MapType,
            __F2__: MapType,
        > UpdateField<Index<0>, __M2__> for __PartialContext<__M1__, __F1__, __F2__> {
            type Value = u64;
            type Mapper = __M1__;
            type Output = __PartialContext<__M2__, __F1__, __F2__>;
            fn update_field(
                self,
                _tag: ::core::marker::PhantomData<Index<0>>,
                value: __M2__::Map<Self::Value>,
            ) -> (__M1__::Map<Self::Value>, Self::Output) {
                (
                    self.0,
                    __PartialContext {
                        0: value,
                        1: self.1,
                        2: self.2,
                    },
                )
            }
        }
        impl<
            __F0__: MapType,
            __M1__: MapType,
            __M2__: MapType,
            __F2__: MapType,
        > UpdateField<Index<1>, __M2__> for __PartialContext<__F0__, __M1__, __F2__> {
            type Value = String;
            type Mapper = __M1__;
            type Output = __PartialContext<__F0__, __M2__, __F2__>;
            fn update_field(
                self,
                _tag: ::core::marker::PhantomData<Index<1>>,
                value: __M2__::Map<Self::Value>,
            ) -> (__M1__::Map<Self::Value>, Self::Output) {
                (
                    self.1,
                    __PartialContext {
                        0: self.0,
                        1: value,
                        2: self.2,
                    },
                )
            }
        }
        impl<
            __F0__: MapType,
            __F1__: MapType,
            __M1__: MapType,
            __M2__: MapType,
        > UpdateField<Index<2>, __M2__> for __PartialContext<__F0__, __F1__, __M1__> {
            type Value = bool;
            type Mapper = __M1__;
            type Output = __PartialContext<__F0__, __F1__, __M2__>;
            fn update_field(
                self,
                _tag: ::core::marker::PhantomData<Index<2>>,
                value: __M2__::Map<Self::Value>,
            ) -> (__M1__::Map<Self::Value>, Self::Output) {
                (
                    self.2,
                    __PartialContext {
                        0: self.0,
                        1: self.1,
                        2: value,
                    },
                )
            }
        }
        impl<__F1__: MapType, __F2__: MapType> HasField<Index<0>>
        for __PartialContext<IsPresent, __F1__, __F2__> {
            type Value = u64;
            fn get_field(&self, tag: ::core::marker::PhantomData<Index<0>>) -> &Self::Value {
                &self.0
            }
        }
        impl<__F0__: MapType, __F2__: MapType> HasField<Index<1>>
        for __PartialContext<__F0__, IsPresent, __F2__> {
            type Value = String;
            fn get_field(&self, tag: ::core::marker::PhantomData<Index<1>>) -> &Self::Value {
                &self.1
            }
        }
        impl<__F0__: MapType, __F1__: MapType> HasField<Index<2>>
        for __PartialContext<__F0__, __F1__, IsPresent> {
            type Value = bool;
            fn get_field(&self, tag: ::core::marker::PhantomData<Index<2>>) -> &Self::Value {
                &self.2
            }
        }
        ")
    }
}
