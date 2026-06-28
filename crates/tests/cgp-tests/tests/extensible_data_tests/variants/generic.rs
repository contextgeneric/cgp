use cgp::core::field::impls::{CanDowncast, CanUpcast};
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_cgp_data;

snapshot_derive_cgp_data! {
    #[derive(CgpData)]
    #[derive(Debug, Eq, PartialEq)]
    pub enum FooBarBazGeneric<Foo, Bar, Baz> {
        Foo(Foo),
        Bar(Bar),
        Baz(Baz),
    }

    expand_foo_bar_baz_generic(output) {
        insta::assert_snapshot!(output, @"
        impl<Foo, Bar, Baz> HasFields for FooBarBazGeneric<Foo, Bar, Baz> {
            type Fields = Either<
                Field<Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>, Foo>,
                Either<
                    Field<Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>, Bar>,
                    Either<Field<Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>, Baz>, Void>,
                >,
            >;
        }
        impl<Foo, Bar, Baz> HasFieldsRef for FooBarBazGeneric<Foo, Bar, Baz> {
            type FieldsRef<'__a> = Either<
                Field<Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>, &'__a Foo>,
                Either<
                    Field<Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>, &'__a Bar>,
                    Either<
                        Field<Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>, &'__a Baz>,
                        Void,
                    >,
                >,
            >
            where
                Self: '__a;
        }
        impl<Foo, Bar, Baz> FromFields for FooBarBazGeneric<Foo, Bar, Baz> {
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
        impl<Foo, Bar, Baz> ToFields for FooBarBazGeneric<Foo, Bar, Baz> {
            fn to_fields(self) -> Self::Fields {
                match self {
                    Self::Foo(field) => Either::Left(field.into()),
                    Self::Bar(field) => Either::Right(Either::Left(field.into())),
                    Self::Baz(field) => Either::Right(Either::Right(Either::Left(field.into()))),
                }
            }
        }
        impl<Foo, Bar, Baz> ToFieldsRef for FooBarBazGeneric<Foo, Bar, Baz> {
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
        impl<Foo, Bar, Baz> FromVariant<Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>>
        for FooBarBazGeneric<Foo, Bar, Baz> {
            type Value = Foo;
            fn from_variant(
                _tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>,
                >,
                value: Self::Value,
            ) -> Self {
                Self::Foo(value)
            }
        }
        impl<Foo, Bar, Baz> FromVariant<Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>>
        for FooBarBazGeneric<Foo, Bar, Baz> {
            type Value = Bar;
            fn from_variant(
                _tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>,
                >,
                value: Self::Value,
            ) -> Self {
                Self::Bar(value)
            }
        }
        impl<Foo, Bar, Baz> FromVariant<Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>>
        for FooBarBazGeneric<Foo, Bar, Baz> {
            type Value = Baz;
            fn from_variant(
                _tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>,
                >,
                value: Self::Value,
            ) -> Self {
                Self::Baz(value)
            }
        }
        pub enum __PartialFooBarBazGeneric<
            Foo,
            Bar,
            Baz,
            __F0__: MapType,
            __F1__: MapType,
            __F2__: MapType,
        > {
            Foo(<__F0__ as MapType>::Map<Foo>),
            Bar(<__F1__ as MapType>::Map<Bar>),
            Baz(<__F2__ as MapType>::Map<Baz>),
        }
        pub enum __PartialRefFooBarBazGeneric<
            '__a__,
            __R__: MapTypeRef,
            Foo: '__a__,
            Bar: '__a__,
            Baz: '__a__,
            __F0__: MapType,
            __F1__: MapType,
            __F2__: MapType,
        > {
            Foo(<__F0__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, Foo>>),
            Bar(<__F1__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, Bar>>),
            Baz(<__F2__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, Baz>>),
        }
        impl<Foo, Bar, Baz, __F0__: MapType, __F1__: MapType, __F2__: MapType> PartialData
        for __PartialFooBarBazGeneric<Foo, Bar, Baz, __F0__, __F1__, __F2__> {
            type Target = FooBarBazGeneric<Foo, Bar, Baz>;
        }
        impl<
            '__a__,
            __R__: MapTypeRef,
            Foo,
            Bar,
            Baz,
            __F0__: MapType,
            __F1__: MapType,
            __F2__: MapType,
        > PartialData
        for __PartialRefFooBarBazGeneric<'__a__, __R__, Foo, Bar, Baz, __F0__, __F1__, __F2__> {
            type Target = FooBarBazGeneric<Foo, Bar, Baz>;
        }
        impl<Foo, Bar, Baz> HasExtractor for FooBarBazGeneric<Foo, Bar, Baz> {
            type Extractor = __PartialFooBarBazGeneric<
                Foo,
                Bar,
                Baz,
                IsPresent,
                IsPresent,
                IsPresent,
            >;
            fn to_extractor(self) -> Self::Extractor {
                match self {
                    Self::Foo(value) => __PartialFooBarBazGeneric::Foo(value),
                    Self::Bar(value) => __PartialFooBarBazGeneric::Bar(value),
                    Self::Baz(value) => __PartialFooBarBazGeneric::Baz(value),
                }
            }
            fn from_extractor(extractor: Self::Extractor) -> Self {
                match extractor {
                    __PartialFooBarBazGeneric::Foo(value) => Self::Foo(value),
                    __PartialFooBarBazGeneric::Bar(value) => Self::Bar(value),
                    __PartialFooBarBazGeneric::Baz(value) => Self::Baz(value),
                }
            }
        }
        impl<Foo, Bar, Baz> HasExtractorRef for FooBarBazGeneric<Foo, Bar, Baz> {
            type ExtractorRef<'a> = __PartialRefFooBarBazGeneric<
                'a,
                IsRef,
                Foo,
                Bar,
                Baz,
                IsPresent,
                IsPresent,
                IsPresent,
            >
            where
                Self: 'a;
            fn extractor_ref<'a>(&'a self) -> Self::ExtractorRef<'a> {
                match self {
                    Self::Foo(value) => __PartialRefFooBarBazGeneric::Foo(value),
                    Self::Bar(value) => __PartialRefFooBarBazGeneric::Bar(value),
                    Self::Baz(value) => __PartialRefFooBarBazGeneric::Baz(value),
                }
            }
        }
        impl<Foo, Bar, Baz> HasExtractorMut for FooBarBazGeneric<Foo, Bar, Baz> {
            type ExtractorMut<'a> = __PartialRefFooBarBazGeneric<
                'a,
                IsMut,
                Foo,
                Bar,
                Baz,
                IsPresent,
                IsPresent,
                IsPresent,
            >
            where
                Self: 'a;
            fn extractor_mut<'a>(&'a mut self) -> Self::ExtractorMut<'a> {
                match self {
                    Self::Foo(value) => __PartialRefFooBarBazGeneric::Foo(value),
                    Self::Bar(value) => __PartialRefFooBarBazGeneric::Bar(value),
                    Self::Baz(value) => __PartialRefFooBarBazGeneric::Baz(value),
                }
            }
        }
        impl<Foo, Bar, Baz> FinalizeExtract
        for __PartialFooBarBazGeneric<Foo, Bar, Baz, IsVoid, IsVoid, IsVoid> {
            fn finalize_extract<__T__>(self) -> __T__ {
                match self {}
            }
        }
        impl<'a, __R__: MapTypeRef, Foo, Bar, Baz> FinalizeExtract
        for __PartialRefFooBarBazGeneric<'a, __R__, Foo, Bar, Baz, IsVoid, IsVoid, IsVoid> {
            fn finalize_extract<__T__>(self) -> __T__ {
                match self {}
            }
        }
        impl<
            Foo,
            Bar,
            Baz,
            __F1__: MapType,
            __F2__: MapType,
        > ExtractField<Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>>
        for __PartialFooBarBazGeneric<Foo, Bar, Baz, IsPresent, __F1__, __F2__> {
            type Value = Foo;
            type Remainder = __PartialFooBarBazGeneric<Foo, Bar, Baz, IsVoid, __F1__, __F2__>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialFooBarBazGeneric::Foo(value) => Ok(value),
                    __PartialFooBarBazGeneric::Bar(value) => {
                        Err(__PartialFooBarBazGeneric::Bar(value))
                    }
                    __PartialFooBarBazGeneric::Baz(value) => {
                        Err(__PartialFooBarBazGeneric::Baz(value))
                    }
                }
            }
        }
        impl<
            Foo,
            Bar,
            Baz,
            __F0__: MapType,
            __F2__: MapType,
        > ExtractField<Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>>
        for __PartialFooBarBazGeneric<Foo, Bar, Baz, __F0__, IsPresent, __F2__> {
            type Value = Bar;
            type Remainder = __PartialFooBarBazGeneric<Foo, Bar, Baz, __F0__, IsVoid, __F2__>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialFooBarBazGeneric::Foo(value) => {
                        Err(__PartialFooBarBazGeneric::Foo(value))
                    }
                    __PartialFooBarBazGeneric::Bar(value) => Ok(value),
                    __PartialFooBarBazGeneric::Baz(value) => {
                        Err(__PartialFooBarBazGeneric::Baz(value))
                    }
                }
            }
        }
        impl<
            Foo,
            Bar,
            Baz,
            __F0__: MapType,
            __F1__: MapType,
        > ExtractField<Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>>
        for __PartialFooBarBazGeneric<Foo, Bar, Baz, __F0__, __F1__, IsPresent> {
            type Value = Baz;
            type Remainder = __PartialFooBarBazGeneric<Foo, Bar, Baz, __F0__, __F1__, IsVoid>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialFooBarBazGeneric::Foo(value) => {
                        Err(__PartialFooBarBazGeneric::Foo(value))
                    }
                    __PartialFooBarBazGeneric::Bar(value) => {
                        Err(__PartialFooBarBazGeneric::Bar(value))
                    }
                    __PartialFooBarBazGeneric::Baz(value) => Ok(value),
                }
            }
        }
        impl<
            '__a__,
            __R__: MapTypeRef,
            Foo,
            Bar,
            Baz,
            __F1__: MapType,
            __F2__: MapType,
        > ExtractField<Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>>
        for __PartialRefFooBarBazGeneric<
            '__a__,
            __R__,
            Foo,
            Bar,
            Baz,
            IsPresent,
            __F1__,
            __F2__,
        > {
            type Value = <__R__ as MapTypeRef>::Map<'__a__, Foo>;
            type Remainder = __PartialRefFooBarBazGeneric<
                '__a__,
                __R__,
                Foo,
                Bar,
                Baz,
                IsVoid,
                __F1__,
                __F2__,
            >;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'F', Chars<'o', Chars<'o', Nil>>>>,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialRefFooBarBazGeneric::Foo(value) => Ok(value),
                    __PartialRefFooBarBazGeneric::Bar(value) => {
                        Err(__PartialRefFooBarBazGeneric::Bar(value))
                    }
                    __PartialRefFooBarBazGeneric::Baz(value) => {
                        Err(__PartialRefFooBarBazGeneric::Baz(value))
                    }
                }
            }
        }
        impl<
            '__a__,
            __R__: MapTypeRef,
            Foo,
            Bar,
            Baz,
            __F0__: MapType,
            __F2__: MapType,
        > ExtractField<Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>>
        for __PartialRefFooBarBazGeneric<
            '__a__,
            __R__,
            Foo,
            Bar,
            Baz,
            __F0__,
            IsPresent,
            __F2__,
        > {
            type Value = <__R__ as MapTypeRef>::Map<'__a__, Bar>;
            type Remainder = __PartialRefFooBarBazGeneric<
                '__a__,
                __R__,
                Foo,
                Bar,
                Baz,
                __F0__,
                IsVoid,
                __F2__,
            >;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'B', Chars<'a', Chars<'r', Nil>>>>,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialRefFooBarBazGeneric::Foo(value) => {
                        Err(__PartialRefFooBarBazGeneric::Foo(value))
                    }
                    __PartialRefFooBarBazGeneric::Bar(value) => Ok(value),
                    __PartialRefFooBarBazGeneric::Baz(value) => {
                        Err(__PartialRefFooBarBazGeneric::Baz(value))
                    }
                }
            }
        }
        impl<
            '__a__,
            __R__: MapTypeRef,
            Foo,
            Bar,
            Baz,
            __F0__: MapType,
            __F1__: MapType,
        > ExtractField<Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>>
        for __PartialRefFooBarBazGeneric<
            '__a__,
            __R__,
            Foo,
            Bar,
            Baz,
            __F0__,
            __F1__,
            IsPresent,
        > {
            type Value = <__R__ as MapTypeRef>::Map<'__a__, Baz>;
            type Remainder = __PartialRefFooBarBazGeneric<
                '__a__,
                __R__,
                Foo,
                Bar,
                Baz,
                __F0__,
                __F1__,
                IsVoid,
            >;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<3, Chars<'B', Chars<'a', Chars<'z', Nil>>>>,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialRefFooBarBazGeneric::Foo(value) => {
                        Err(__PartialRefFooBarBazGeneric::Foo(value))
                    }
                    __PartialRefFooBarBazGeneric::Bar(value) => {
                        Err(__PartialRefFooBarBazGeneric::Bar(value))
                    }
                    __PartialRefFooBarBazGeneric::Baz(value) => Ok(value),
                }
            }
        }
        ")
    }
}

#[derive(Debug, Eq, PartialEq, CgpData)]
pub enum FooBarGeneric<Foo, Bar> {
    Foo(Foo),
    Bar(Bar),
}

pub type FooBarBaz = FooBarBazGeneric<u64, String, bool>;
pub type FooBar = FooBarGeneric<u64, String>;

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
}
