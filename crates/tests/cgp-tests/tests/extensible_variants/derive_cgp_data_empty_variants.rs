//! `#[derive(CgpData)]` on an enum mixing a newtype variant with the three forms
//! of a variant with no fields: `Closed`, `Paused()`, and `Archived {}`. Each
//! empty variant carries the payload `Nil` throughout: `HasFields` lists it as
//! `Field<Tag, Nil>`, `FromVariant` takes a `Nil` and builds the variant with
//! `{}`, and the extractors match it with `{ .. }`, both braced forms working for
//! all three. The owned extractor holds `Nil`, the shared one a promoted `&Nil`,
//! and the mutable one a leaked `Box<Nil>`, since Rust does not promote
//! `&mut Nil`. That `Box` is written bare and resolved where the derive is used.
//!
//! This concept owns the variant expansion of `#[derive(CgpData)]`; this file is
//! the empty-variant snapshot, and `empty_variants.rs` exercises its behavior.
//!
//! See cgp-knowledge-base/cgp/reference/derives/derive_cgp_variant.md and
//! cgp-knowledge-base/cgp/implementation/entrypoints/derive_extract_field.md.

use cgp_macro_test_util::snapshot_derive_cgp_data;

snapshot_derive_cgp_data! {
    #[derive(CgpData)]
    pub enum Status {
        Active(u64),
        Closed,
        Paused(),
        Archived {},
    }

    expand_status(output) {
        insta::assert_snapshot!(output, @"
        impl HasFields for Status {
            type Fields = Either<
                Field<
                    Symbol<
                        6,
                        Chars<
                            'A',
                            Chars<'c', Chars<'t', Chars<'i', Chars<'v', Chars<'e', Nil>>>>>,
                        >,
                    >,
                    u64,
                >,
                Either<
                    Field<
                        Symbol<
                            6,
                            Chars<
                                'C',
                                Chars<'l', Chars<'o', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>,
                            >,
                        >,
                        Nil,
                    >,
                    Either<
                        Field<
                            Symbol<
                                6,
                                Chars<
                                    'P',
                                    Chars<
                                        'a',
                                        Chars<'u', Chars<'s', Chars<'e', Chars<'d', Nil>>>>,
                                    >,
                                >,
                            >,
                            Nil,
                        >,
                        Either<
                            Field<
                                Symbol<
                                    8,
                                    Chars<
                                        'A',
                                        Chars<
                                            'r',
                                            Chars<
                                                'c',
                                                Chars<
                                                    'h',
                                                    Chars<'i', Chars<'v', Chars<'e', Chars<'d', Nil>>>>,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                                Nil,
                            >,
                            Void,
                        >,
                    >,
                >,
            >;
        }
        impl HasFieldsRef for Status {
            type FieldsRef<'__a> = Either<
                Field<
                    Symbol<
                        6,
                        Chars<
                            'A',
                            Chars<'c', Chars<'t', Chars<'i', Chars<'v', Chars<'e', Nil>>>>>,
                        >,
                    >,
                    &'__a u64,
                >,
                Either<
                    Field<
                        Symbol<
                            6,
                            Chars<
                                'C',
                                Chars<'l', Chars<'o', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>,
                            >,
                        >,
                        Nil,
                    >,
                    Either<
                        Field<
                            Symbol<
                                6,
                                Chars<
                                    'P',
                                    Chars<
                                        'a',
                                        Chars<'u', Chars<'s', Chars<'e', Chars<'d', Nil>>>>,
                                    >,
                                >,
                            >,
                            Nil,
                        >,
                        Either<
                            Field<
                                Symbol<
                                    8,
                                    Chars<
                                        'A',
                                        Chars<
                                            'r',
                                            Chars<
                                                'c',
                                                Chars<
                                                    'h',
                                                    Chars<'i', Chars<'v', Chars<'e', Chars<'d', Nil>>>>,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                                Nil,
                            >,
                            Void,
                        >,
                    >,
                >,
            >
            where
                Self: '__a;
        }
        impl FromFields for Status {
            fn from_fields(rest: Self::Fields) -> Self {
                match rest {
                    Either::Left(field) => {
                        let field = field.value;
                        Self::Active(field)
                    }
                    Either::Right(rest) => {
                        match rest {
                            Either::Left(field) => {
                                let Nil = field.value;
                                Self::Closed
                            }
                            Either::Right(rest) => {
                                match rest {
                                    Either::Left(field) => {
                                        let Nil = field.value;
                                        Self::Paused()
                                    }
                                    Either::Right(rest) => {
                                        match rest {
                                            Either::Left(field) => {
                                                let Nil = field.value;
                                                Self::Archived {}
                                            }
                                            Either::Right(rest) => match rest {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        impl ToFields for Status {
            fn to_fields(self) -> Self::Fields {
                match self {
                    Self::Active(field) => Either::Left(field.into()),
                    Self::Closed => Either::Right(Either::Left(Nil.into())),
                    Self::Paused() => Either::Right(Either::Right(Either::Left(Nil.into()))),
                    Self::Archived {} => {
                        Either::Right(Either::Right(Either::Right(Either::Left(Nil.into()))))
                    }
                }
            }
        }
        impl ToFieldsRef for Status {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                match self {
                    Self::Active(field) => Either::Left(field.into()),
                    Self::Closed => Either::Right(Either::Left(Nil.into())),
                    Self::Paused() => Either::Right(Either::Right(Either::Left(Nil.into()))),
                    Self::Archived {} => {
                        Either::Right(Either::Right(Either::Right(Either::Left(Nil.into()))))
                    }
                }
            }
        }
        impl FromVariant<
            Symbol<
                6,
                Chars<'A', Chars<'c', Chars<'t', Chars<'i', Chars<'v', Chars<'e', Nil>>>>>>,
            >,
        > for Status {
            type Value = u64;
            fn from_variant(
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'A',
                            Chars<'c', Chars<'t', Chars<'i', Chars<'v', Chars<'e', Nil>>>>>,
                        >,
                    >,
                >,
                value: Self::Value,
            ) -> Self {
                Self::Active(value)
            }
        }
        impl FromVariant<
            Symbol<
                6,
                Chars<'C', Chars<'l', Chars<'o', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>>,
            >,
        > for Status {
            type Value = Nil;
            fn from_variant(
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'C',
                            Chars<'l', Chars<'o', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                >,
                _: Self::Value,
            ) -> Self {
                Self::Closed {}
            }
        }
        impl FromVariant<
            Symbol<
                6,
                Chars<'P', Chars<'a', Chars<'u', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>>,
            >,
        > for Status {
            type Value = Nil;
            fn from_variant(
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'P',
                            Chars<'a', Chars<'u', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                >,
                _: Self::Value,
            ) -> Self {
                Self::Paused {}
            }
        }
        impl FromVariant<
            Symbol<
                8,
                Chars<
                    'A',
                    Chars<
                        'r',
                        Chars<
                            'c',
                            Chars<'h', Chars<'i', Chars<'v', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                >,
            >,
        > for Status {
            type Value = Nil;
            fn from_variant(
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        8,
                        Chars<
                            'A',
                            Chars<
                                'r',
                                Chars<
                                    'c',
                                    Chars<
                                        'h',
                                        Chars<'i', Chars<'v', Chars<'e', Chars<'d', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
                _: Self::Value,
            ) -> Self {
                Self::Archived {}
            }
        }
        pub enum __PartialStatus<
            __F0__: MapType,
            __F1__: MapType,
            __F2__: MapType,
            __F3__: MapType,
        > {
            Active(<__F0__ as MapType>::Map<u64>),
            Closed(<__F1__ as MapType>::Map<Nil>),
            Paused(<__F2__ as MapType>::Map<Nil>),
            Archived(<__F3__ as MapType>::Map<Nil>),
        }
        pub enum __PartialRefStatus<
            '__a__,
            __R__: MapTypeRef,
            __F0__: MapType,
            __F1__: MapType,
            __F2__: MapType,
            __F3__: MapType,
        > {
            Active(<__F0__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, u64>>),
            Closed(<__F1__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, Nil>>),
            Paused(<__F2__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, Nil>>),
            Archived(<__F3__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, Nil>>),
        }
        impl<__F0__: MapType, __F1__: MapType, __F2__: MapType, __F3__: MapType> PartialData
        for __PartialStatus<__F0__, __F1__, __F2__, __F3__> {
            type Target = Status;
        }
        impl<
            '__a__,
            __R__: MapTypeRef,
            __F0__: MapType,
            __F1__: MapType,
            __F2__: MapType,
            __F3__: MapType,
        > PartialData for __PartialRefStatus<'__a__, __R__, __F0__, __F1__, __F2__, __F3__> {
            type Target = Status;
        }
        impl HasExtractor for Status {
            type Extractor = __PartialStatus<IsPresent, IsPresent, IsPresent, IsPresent>;
            fn to_extractor(self) -> Self::Extractor {
                match self {
                    Self::Active(value) => __PartialStatus::Active(value),
                    Self::Closed { .. } => __PartialStatus::Closed(Nil),
                    Self::Paused { .. } => __PartialStatus::Paused(Nil),
                    Self::Archived { .. } => __PartialStatus::Archived(Nil),
                }
            }
            fn from_extractor(extractor: Self::Extractor) -> Self {
                match extractor {
                    __PartialStatus::Active(value) => Self::Active(value),
                    __PartialStatus::Closed(_) => Self::Closed {},
                    __PartialStatus::Paused(_) => Self::Paused {},
                    __PartialStatus::Archived(_) => Self::Archived {},
                }
            }
        }
        impl HasExtractorRef for Status {
            type ExtractorRef<'__a__> = __PartialRefStatus<
                '__a__,
                IsRef,
                IsPresent,
                IsPresent,
                IsPresent,
                IsPresent,
            >
            where
                Self: '__a__;
            fn extractor_ref<'__a__>(&'__a__ self) -> Self::ExtractorRef<'__a__> {
                match self {
                    Self::Active(value) => __PartialRefStatus::Active(value),
                    Self::Closed { .. } => __PartialRefStatus::Closed(&Nil),
                    Self::Paused { .. } => __PartialRefStatus::Paused(&Nil),
                    Self::Archived { .. } => __PartialRefStatus::Archived(&Nil),
                }
            }
        }
        impl HasExtractorMut for Status {
            type ExtractorMut<'__a__> = __PartialRefStatus<
                '__a__,
                IsMut,
                IsPresent,
                IsPresent,
                IsPresent,
                IsPresent,
            >
            where
                Self: '__a__;
            fn extractor_mut<'__a__>(&'__a__ mut self) -> Self::ExtractorMut<'__a__> {
                match self {
                    Self::Active(value) => __PartialRefStatus::Active(value),
                    Self::Closed { .. } => __PartialRefStatus::Closed(Box::leak(Box::new(Nil))),
                    Self::Paused { .. } => __PartialRefStatus::Paused(Box::leak(Box::new(Nil))),
                    Self::Archived { .. } => {
                        __PartialRefStatus::Archived(Box::leak(Box::new(Nil)))
                    }
                }
            }
        }
        impl FinalizeExtract for __PartialStatus<IsVoid, IsVoid, IsVoid, IsVoid> {
            fn finalize_extract<__T__>(self) -> __T__ {
                match self {}
            }
        }
        impl<'__a__, __R__: MapTypeRef> FinalizeExtract
        for __PartialRefStatus<'__a__, __R__, IsVoid, IsVoid, IsVoid, IsVoid> {
            fn finalize_extract<__T__>(self) -> __T__ {
                match self {}
            }
        }
        impl<
            __F1__: MapType,
            __F2__: MapType,
            __F3__: MapType,
        > ExtractField<
            Symbol<
                6,
                Chars<'A', Chars<'c', Chars<'t', Chars<'i', Chars<'v', Chars<'e', Nil>>>>>>,
            >,
        > for __PartialStatus<IsPresent, __F1__, __F2__, __F3__> {
            type Value = u64;
            type Remainder = __PartialStatus<IsVoid, __F1__, __F2__, __F3__>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'A',
                            Chars<'c', Chars<'t', Chars<'i', Chars<'v', Chars<'e', Nil>>>>>,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialStatus::Active(value) => Ok(value),
                    __PartialStatus::Closed(value) => Err(__PartialStatus::Closed(value)),
                    __PartialStatus::Paused(value) => Err(__PartialStatus::Paused(value)),
                    __PartialStatus::Archived(value) => Err(__PartialStatus::Archived(value)),
                }
            }
        }
        impl<
            __F0__: MapType,
            __F2__: MapType,
            __F3__: MapType,
        > ExtractField<
            Symbol<
                6,
                Chars<'C', Chars<'l', Chars<'o', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>>,
            >,
        > for __PartialStatus<__F0__, IsPresent, __F2__, __F3__> {
            type Value = Nil;
            type Remainder = __PartialStatus<__F0__, IsVoid, __F2__, __F3__>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'C',
                            Chars<'l', Chars<'o', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialStatus::Active(value) => Err(__PartialStatus::Active(value)),
                    __PartialStatus::Closed(value) => Ok(value),
                    __PartialStatus::Paused(value) => Err(__PartialStatus::Paused(value)),
                    __PartialStatus::Archived(value) => Err(__PartialStatus::Archived(value)),
                }
            }
        }
        impl<
            __F0__: MapType,
            __F1__: MapType,
            __F3__: MapType,
        > ExtractField<
            Symbol<
                6,
                Chars<'P', Chars<'a', Chars<'u', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>>,
            >,
        > for __PartialStatus<__F0__, __F1__, IsPresent, __F3__> {
            type Value = Nil;
            type Remainder = __PartialStatus<__F0__, __F1__, IsVoid, __F3__>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'P',
                            Chars<'a', Chars<'u', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialStatus::Active(value) => Err(__PartialStatus::Active(value)),
                    __PartialStatus::Closed(value) => Err(__PartialStatus::Closed(value)),
                    __PartialStatus::Paused(value) => Ok(value),
                    __PartialStatus::Archived(value) => Err(__PartialStatus::Archived(value)),
                }
            }
        }
        impl<
            __F0__: MapType,
            __F1__: MapType,
            __F2__: MapType,
        > ExtractField<
            Symbol<
                8,
                Chars<
                    'A',
                    Chars<
                        'r',
                        Chars<
                            'c',
                            Chars<'h', Chars<'i', Chars<'v', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                >,
            >,
        > for __PartialStatus<__F0__, __F1__, __F2__, IsPresent> {
            type Value = Nil;
            type Remainder = __PartialStatus<__F0__, __F1__, __F2__, IsVoid>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        8,
                        Chars<
                            'A',
                            Chars<
                                'r',
                                Chars<
                                    'c',
                                    Chars<
                                        'h',
                                        Chars<'i', Chars<'v', Chars<'e', Chars<'d', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialStatus::Active(value) => Err(__PartialStatus::Active(value)),
                    __PartialStatus::Closed(value) => Err(__PartialStatus::Closed(value)),
                    __PartialStatus::Paused(value) => Err(__PartialStatus::Paused(value)),
                    __PartialStatus::Archived(value) => Ok(value),
                }
            }
        }
        impl<
            '__a__,
            __R__: MapTypeRef,
            __F1__: MapType,
            __F2__: MapType,
            __F3__: MapType,
        > ExtractField<
            Symbol<
                6,
                Chars<'A', Chars<'c', Chars<'t', Chars<'i', Chars<'v', Chars<'e', Nil>>>>>>,
            >,
        > for __PartialRefStatus<'__a__, __R__, IsPresent, __F1__, __F2__, __F3__> {
            type Value = <__R__ as MapTypeRef>::Map<'__a__, u64>;
            type Remainder = __PartialRefStatus<'__a__, __R__, IsVoid, __F1__, __F2__, __F3__>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'A',
                            Chars<'c', Chars<'t', Chars<'i', Chars<'v', Chars<'e', Nil>>>>>,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialRefStatus::Active(value) => Ok(value),
                    __PartialRefStatus::Closed(value) => Err(__PartialRefStatus::Closed(value)),
                    __PartialRefStatus::Paused(value) => Err(__PartialRefStatus::Paused(value)),
                    __PartialRefStatus::Archived(value) => {
                        Err(__PartialRefStatus::Archived(value))
                    }
                }
            }
        }
        impl<
            '__a__,
            __R__: MapTypeRef,
            __F0__: MapType,
            __F2__: MapType,
            __F3__: MapType,
        > ExtractField<
            Symbol<
                6,
                Chars<'C', Chars<'l', Chars<'o', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>>,
            >,
        > for __PartialRefStatus<'__a__, __R__, __F0__, IsPresent, __F2__, __F3__> {
            type Value = <__R__ as MapTypeRef>::Map<'__a__, Nil>;
            type Remainder = __PartialRefStatus<'__a__, __R__, __F0__, IsVoid, __F2__, __F3__>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'C',
                            Chars<'l', Chars<'o', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialRefStatus::Active(value) => Err(__PartialRefStatus::Active(value)),
                    __PartialRefStatus::Closed(value) => Ok(value),
                    __PartialRefStatus::Paused(value) => Err(__PartialRefStatus::Paused(value)),
                    __PartialRefStatus::Archived(value) => {
                        Err(__PartialRefStatus::Archived(value))
                    }
                }
            }
        }
        impl<
            '__a__,
            __R__: MapTypeRef,
            __F0__: MapType,
            __F1__: MapType,
            __F3__: MapType,
        > ExtractField<
            Symbol<
                6,
                Chars<'P', Chars<'a', Chars<'u', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>>,
            >,
        > for __PartialRefStatus<'__a__, __R__, __F0__, __F1__, IsPresent, __F3__> {
            type Value = <__R__ as MapTypeRef>::Map<'__a__, Nil>;
            type Remainder = __PartialRefStatus<'__a__, __R__, __F0__, __F1__, IsVoid, __F3__>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'P',
                            Chars<'a', Chars<'u', Chars<'s', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialRefStatus::Active(value) => Err(__PartialRefStatus::Active(value)),
                    __PartialRefStatus::Closed(value) => Err(__PartialRefStatus::Closed(value)),
                    __PartialRefStatus::Paused(value) => Ok(value),
                    __PartialRefStatus::Archived(value) => {
                        Err(__PartialRefStatus::Archived(value))
                    }
                }
            }
        }
        impl<
            '__a__,
            __R__: MapTypeRef,
            __F0__: MapType,
            __F1__: MapType,
            __F2__: MapType,
        > ExtractField<
            Symbol<
                8,
                Chars<
                    'A',
                    Chars<
                        'r',
                        Chars<
                            'c',
                            Chars<'h', Chars<'i', Chars<'v', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                >,
            >,
        > for __PartialRefStatus<'__a__, __R__, __F0__, __F1__, __F2__, IsPresent> {
            type Value = <__R__ as MapTypeRef>::Map<'__a__, Nil>;
            type Remainder = __PartialRefStatus<'__a__, __R__, __F0__, __F1__, __F2__, IsVoid>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        8,
                        Chars<
                            'A',
                            Chars<
                                'r',
                                Chars<
                                    'c',
                                    Chars<
                                        'h',
                                        Chars<'i', Chars<'v', Chars<'e', Chars<'d', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialRefStatus::Active(value) => Err(__PartialRefStatus::Active(value)),
                    __PartialRefStatus::Closed(value) => Err(__PartialRefStatus::Closed(value)),
                    __PartialRefStatus::Paused(value) => Err(__PartialRefStatus::Paused(value)),
                    __PartialRefStatus::Archived(value) => Ok(value),
                }
            }
        }
        ")
    }
}
