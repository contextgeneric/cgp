//! `#[derive(CgpData)]` on a concrete enum: the full variant machinery. Beyond
//! the `HasFields` field list, the derive emits the `FromVariant` constructors,
//! the `__Partial*` extractor enums, and the `HasExtractor`/`ExtractField`/
//! `FinalizeExtract` impls that deconstruct a value one variant at a time,
//! narrowing the remainder's type as each present variant is removed.
//!
//! This concept owns the variant expansion of `#[derive(CgpData)]`; this file
//! is the canonical snapshot for a concrete (non-generic) enum. It also
//! exercises the extractor at runtime and the structural casts
//! (`CanUpcast`/`CanDowncast`) between enum shapes; the auxiliary enums are
//! plain derives, since the primary snapshot already pins the expansion.
//!
//! See docs/reference/derives/derive_cgp_data.md,
//! docs/reference/derives/derive_from_variant.md, and
//! docs/reference/derives/derive_extract_field.md.

use core::marker::PhantomData;

use cgp::core::field::impls::{CanDowncast, CanDowncastFields, CanUpcast};
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_cgp_data;

snapshot_derive_cgp_data! {
    #[derive(CgpData)]
    #[derive(Debug, Eq, PartialEq)]
    pub enum FooBarBaz {
        Foo(u64),
        Bar(String),
        Baz(bool),
    }

    expand_foo_bar_baz(output) {
            insta::assert_snapshot!(output, @"
            impl HasFields for FooBarBaz {
                type Fields = Either<
                    Field<Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>, u64>,
                    Either<
                        Field<Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>, String>,
                        Either<Field<Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>, bool>, Void>,
                    >,
                >;
            }
            impl HasFieldsRef for FooBarBaz {
                type FieldsRef<'__a> = Either<
                    Field<Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>, &'__a u64>,
                    Either<
                        Field<Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>, &'__a String>,
                        Either<
                            Field<Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>, &'__a bool>,
                            Void,
                        >,
                    >,
                >
                where
                    Self: '__a;
            }
            impl FromFields for FooBarBaz {
                fn from_fields(rest: Self::Fields) -> Self {
                    match rest {
                        Either::Left(field) => {
                            let field = field.value;
                            Self::Foo(field)
                        }
                        Either::Right(rest) => {
                            match rest {
                                Either::Left(field) => {
                                    let field = field.value;
                                    Self::Bar(field)
                                }
                                Either::Right(rest) => {
                                    match rest {
                                        Either::Left(field) => {
                                            let field = field.value;
                                            Self::Baz(field)
                                        }
                                        Either::Right(rest) => match rest {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
            impl ToFields for FooBarBaz {
                fn to_fields(self) -> Self::Fields {
                    match self {
                        Self::Foo(field) => Either::Left(field.into()),
                        Self::Bar(field) => Either::Right(Either::Left(field.into())),
                        Self::Baz(field) => Either::Right(Either::Right(Either::Left(field.into()))),
                    }
                }
            }
            impl ToFieldsRef for FooBarBaz {
                fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
                where
                    Self: '__a,
                {
                    match self {
                        Self::Foo(field) => Either::Left(field.into()),
                        Self::Bar(field) => Either::Right(Either::Left(field.into())),
                        Self::Baz(field) => Either::Right(Either::Right(Either::Left(field.into()))),
                    }
                }
            }
            impl FromVariant<Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>> for FooBarBaz {
                type Value = u64;
                fn from_variant(
                    _tag: ::core::marker::PhantomData<
                        Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>,
                    >,
                    value: Self::Value,
                ) -> Self {
                    Self::Foo(value)
                }
            }
            impl FromVariant<Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>> for FooBarBaz {
                type Value = String;
                fn from_variant(
                    _tag: ::core::marker::PhantomData<
                        Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>,
                    >,
                    value: Self::Value,
                ) -> Self {
                    Self::Bar(value)
                }
            }
            impl FromVariant<Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>> for FooBarBaz {
                type Value = bool;
                fn from_variant(
                    _tag: ::core::marker::PhantomData<
                        Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>,
                    >,
                    value: Self::Value,
                ) -> Self {
                    Self::Baz(value)
                }
            }
            pub enum __PartialFooBarBaz<__F0__: MapType, __F1__: MapType, __F2__: MapType> {
                Foo(<__F0__ as MapType>::Map<u64>),
                Bar(<__F1__ as MapType>::Map<String>),
                Baz(<__F2__ as MapType>::Map<bool>),
            }
            pub enum __PartialRefFooBarBaz<
                '__a__,
                __R__: MapTypeRef,
                __F0__: MapType,
                __F1__: MapType,
                __F2__: MapType,
            > {
                Foo(<__F0__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, u64>>),
                Bar(<__F1__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, String>>),
                Baz(<__F2__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, bool>>),
            }
            impl<__F0__: MapType, __F1__: MapType, __F2__: MapType> PartialData
            for __PartialFooBarBaz<__F0__, __F1__, __F2__> {
                type Target = FooBarBaz;
            }
            impl<
                '__a__,
                __R__: MapTypeRef,
                __F0__: MapType,
                __F1__: MapType,
                __F2__: MapType,
            > PartialData for __PartialRefFooBarBaz<'__a__, __R__, __F0__, __F1__, __F2__> {
                type Target = FooBarBaz;
            }
            impl HasExtractor for FooBarBaz {
                type Extractor = __PartialFooBarBaz<IsPresent, IsPresent, IsPresent>;
                fn to_extractor(self) -> Self::Extractor {
                    match self {
                        Self::Foo(value) => __PartialFooBarBaz::Foo(value),
                        Self::Bar(value) => __PartialFooBarBaz::Bar(value),
                        Self::Baz(value) => __PartialFooBarBaz::Baz(value),
                    }
                }
                fn from_extractor(extractor: Self::Extractor) -> Self {
                    match extractor {
                        __PartialFooBarBaz::Foo(value) => Self::Foo(value),
                        __PartialFooBarBaz::Bar(value) => Self::Bar(value),
                        __PartialFooBarBaz::Baz(value) => Self::Baz(value),
                    }
                }
            }
            impl HasExtractorRef for FooBarBaz {
                type ExtractorRef<'a> = __PartialRefFooBarBaz<
                    'a,
                    IsRef,
                    IsPresent,
                    IsPresent,
                    IsPresent,
                >
                where
                    Self: 'a;
                fn extractor_ref<'a>(&'a self) -> Self::ExtractorRef<'a> {
                    match self {
                        Self::Foo(value) => __PartialRefFooBarBaz::Foo(value),
                        Self::Bar(value) => __PartialRefFooBarBaz::Bar(value),
                        Self::Baz(value) => __PartialRefFooBarBaz::Baz(value),
                    }
                }
            }
            impl HasExtractorMut for FooBarBaz {
                type ExtractorMut<'a> = __PartialRefFooBarBaz<
                    'a,
                    IsMut,
                    IsPresent,
                    IsPresent,
                    IsPresent,
                >
                where
                    Self: 'a;
                fn extractor_mut<'a>(&'a mut self) -> Self::ExtractorMut<'a> {
                    match self {
                        Self::Foo(value) => __PartialRefFooBarBaz::Foo(value),
                        Self::Bar(value) => __PartialRefFooBarBaz::Bar(value),
                        Self::Baz(value) => __PartialRefFooBarBaz::Baz(value),
                    }
                }
            }
            impl FinalizeExtract for __PartialFooBarBaz<IsVoid, IsVoid, IsVoid> {
                fn finalize_extract<__T__>(self) -> __T__ {
                    match self {}
                }
            }
            impl<'a, __R__: MapTypeRef> FinalizeExtract
            for __PartialRefFooBarBaz<'a, __R__, IsVoid, IsVoid, IsVoid> {
                fn finalize_extract<__T__>(self) -> __T__ {
                    match self {}
                }
            }
            impl<
                __F1__: MapType,
                __F2__: MapType,
            > ExtractField<Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>>
            for __PartialFooBarBaz<IsPresent, __F1__, __F2__> {
                type Value = u64;
                type Remainder = __PartialFooBarBaz<IsVoid, __F1__, __F2__>;
                fn extract_field(
                    self,
                    _tag: ::core::marker::PhantomData<
                        Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>,
                    >,
                ) -> Result<Self::Value, Self::Remainder> {
                    match self {
                        __PartialFooBarBaz::Foo(value) => Ok(value),
                        __PartialFooBarBaz::Bar(value) => Err(__PartialFooBarBaz::Bar(value)),
                        __PartialFooBarBaz::Baz(value) => Err(__PartialFooBarBaz::Baz(value)),
                    }
                }
            }
            impl<
                __F0__: MapType,
                __F2__: MapType,
            > ExtractField<Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>>
            for __PartialFooBarBaz<__F0__, IsPresent, __F2__> {
                type Value = String;
                type Remainder = __PartialFooBarBaz<__F0__, IsVoid, __F2__>;
                fn extract_field(
                    self,
                    _tag: ::core::marker::PhantomData<
                        Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>,
                    >,
                ) -> Result<Self::Value, Self::Remainder> {
                    match self {
                        __PartialFooBarBaz::Foo(value) => Err(__PartialFooBarBaz::Foo(value)),
                        __PartialFooBarBaz::Bar(value) => Ok(value),
                        __PartialFooBarBaz::Baz(value) => Err(__PartialFooBarBaz::Baz(value)),
                    }
                }
            }
            impl<
                __F0__: MapType,
                __F1__: MapType,
            > ExtractField<Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>>
            for __PartialFooBarBaz<__F0__, __F1__, IsPresent> {
                type Value = bool;
                type Remainder = __PartialFooBarBaz<__F0__, __F1__, IsVoid>;
                fn extract_field(
                    self,
                    _tag: ::core::marker::PhantomData<
                        Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>,
                    >,
                ) -> Result<Self::Value, Self::Remainder> {
                    match self {
                        __PartialFooBarBaz::Foo(value) => Err(__PartialFooBarBaz::Foo(value)),
                        __PartialFooBarBaz::Bar(value) => Err(__PartialFooBarBaz::Bar(value)),
                        __PartialFooBarBaz::Baz(value) => Ok(value),
                    }
                }
            }
            impl<
                '__a__,
                __R__: MapTypeRef,
                __F1__: MapType,
                __F2__: MapType,
            > ExtractField<Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>>
            for __PartialRefFooBarBaz<'__a__, __R__, IsPresent, __F1__, __F2__> {
                type Value = <__R__ as MapTypeRef>::Map<'__a__, u64>;
                type Remainder = __PartialRefFooBarBaz<'__a__, __R__, IsVoid, __F1__, __F2__>;
                fn extract_field(
                    self,
                    _tag: ::core::marker::PhantomData<
                        Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>,
                    >,
                ) -> Result<Self::Value, Self::Remainder> {
                    match self {
                        __PartialRefFooBarBaz::Foo(value) => Ok(value),
                        __PartialRefFooBarBaz::Bar(value) => Err(__PartialRefFooBarBaz::Bar(value)),
                        __PartialRefFooBarBaz::Baz(value) => Err(__PartialRefFooBarBaz::Baz(value)),
                    }
                }
            }
            impl<
                '__a__,
                __R__: MapTypeRef,
                __F0__: MapType,
                __F2__: MapType,
            > ExtractField<Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>>
            for __PartialRefFooBarBaz<'__a__, __R__, __F0__, IsPresent, __F2__> {
                type Value = <__R__ as MapTypeRef>::Map<'__a__, String>;
                type Remainder = __PartialRefFooBarBaz<'__a__, __R__, __F0__, IsVoid, __F2__>;
                fn extract_field(
                    self,
                    _tag: ::core::marker::PhantomData<
                        Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>,
                    >,
                ) -> Result<Self::Value, Self::Remainder> {
                    match self {
                        __PartialRefFooBarBaz::Foo(value) => Err(__PartialRefFooBarBaz::Foo(value)),
                        __PartialRefFooBarBaz::Bar(value) => Ok(value),
                        __PartialRefFooBarBaz::Baz(value) => Err(__PartialRefFooBarBaz::Baz(value)),
                    }
                }
            }
            impl<
                '__a__,
                __R__: MapTypeRef,
                __F0__: MapType,
                __F1__: MapType,
            > ExtractField<Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>>
            for __PartialRefFooBarBaz<'__a__, __R__, __F0__, __F1__, IsPresent> {
                type Value = <__R__ as MapTypeRef>::Map<'__a__, bool>;
                type Remainder = __PartialRefFooBarBaz<'__a__, __R__, __F0__, __F1__, IsVoid>;
                fn extract_field(
                    self,
                    _tag: ::core::marker::PhantomData<
                        Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>,
                    >,
                ) -> Result<Self::Value, Self::Remainder> {
                    match self {
                        __PartialRefFooBarBaz::Foo(value) => Err(__PartialRefFooBarBaz::Foo(value)),
                        __PartialRefFooBarBaz::Bar(value) => Err(__PartialRefFooBarBaz::Bar(value)),
                        __PartialRefFooBarBaz::Baz(value) => Ok(value),
                    }
                }
            }
            ")
    }
}

// Auxiliary enums, used as cast targets. These are plain derives: the primary
// snapshot above already pins the `#[derive(CgpData)]` expansion.
#[derive(Debug, Eq, PartialEq, CgpData)]
pub enum FooBar {
    Foo(u64),
    Bar(String),
}

#[derive(Debug, Eq, PartialEq, CgpData)]
pub enum Baz {
    Baz(bool),
}

#[derive(Debug, Eq, PartialEq, CgpData)]
pub enum BazBarFoo {
    Baz(bool),
    Bar(String),
    Foo(u64),
}

// Walk the extractor by hand: each `extract_field` pulls out one variant or
// hands back a remainder whose type no longer contains it, until the exhausted
// remainder is closed off with `finalize_extract`.
fn context_to_string(context: FooBarBaz) -> String {
    match context
        .extractor_ref()
        .extract_field(PhantomData::<Symbol!("Foo")>)
    {
        Ok(value) => value.to_string(),
        Err(remainder) => match remainder.extract_field(PhantomData::<Symbol!("Bar")>) {
            Ok(value) => value.to_string(),
            Err(remainder) => match remainder.extract_field(PhantomData::<Symbol!("Baz")>) {
                Ok(value) => value.to_string(),
                Err(remainder) => remainder.finalize_extract(),
            },
        },
    }
}

#[test]
fn test_basic_extractor() {
    assert_eq!(context_to_string(FooBarBaz::Foo(1)), "1");
    assert_eq!(
        context_to_string(FooBarBaz::Bar("hello".to_owned())),
        "hello"
    );
    assert_eq!(context_to_string(FooBarBaz::Baz(true)), "true");
}

#[test]
fn test_upcast() {
    assert_eq!(
        FooBar::Foo(1).upcast(PhantomData::<FooBarBaz>),
        FooBarBaz::Foo(1)
    );

    assert_eq!(
        FooBar::Bar("hello".to_owned()).upcast(PhantomData::<FooBarBaz>),
        FooBarBaz::Bar("hello".to_owned())
    );
}

#[test]
fn test_downcast() {
    assert_eq!(
        FooBarBaz::Foo(1).downcast(PhantomData::<FooBar>).ok(),
        Some(FooBar::Foo(1))
    );

    assert_eq!(
        FooBarBaz::Bar("hello".to_owned())
            .downcast(PhantomData::<FooBar>)
            .ok(),
        Some(FooBar::Bar("hello".to_owned()))
    );

    assert_eq!(
        FooBarBaz::Baz(true).downcast(PhantomData::<FooBar>).ok(),
        None
    );

    {
        let remainder = FooBarBaz::Baz(true)
            .downcast(PhantomData::<FooBar>)
            .unwrap_err();
        assert_eq!(
            remainder.downcast_fields(PhantomData::<Baz>).ok(),
            Some(Baz::Baz(true))
        );
    }

    assert_eq!(
        FooBarBaz::Foo(1).downcast(PhantomData::<BazBarFoo>).ok(),
        Some(BazBarFoo::Foo(1))
    );

    assert_eq!(
        FooBarBaz::Bar("hello".to_owned())
            .downcast(PhantomData::<BazBarFoo>)
            .ok(),
        Some(BazBarFoo::Bar("hello".to_owned()))
    );

    assert_eq!(
        FooBarBaz::Baz(true).downcast(PhantomData::<BazBarFoo>).ok(),
        Some(BazBarFoo::Baz(true))
    );
}
