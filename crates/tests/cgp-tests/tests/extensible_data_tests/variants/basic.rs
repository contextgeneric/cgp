use core::convert::Infallible;
use core::fmt::{Debug, Display};
use core::marker::PhantomData;

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::core::field::impls::{CanDowncast, CanDowncastFields, CanUpcast};
use cgp::extra::dispatch::{
    DowncastAndHandle, ExtractFieldAndHandle, HandleFieldValue, MatchWithFieldHandlers,
    MatchWithHandlers, MatchWithValueHandlersRef,
};
use cgp::extra::handler::{
    Computer, ComputerComponent, ComputerRef, ComputerRefComponent, PromoteAsync,
};
use cgp::prelude::*;
use cgp_macro_test_util::{
    snapshot_cgp_new_provider, snapshot_delegate_components, snapshot_derive_cgp_data,
};
use futures::executor::block_on;

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

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            ErrorTypeProviderComponent: UseType<Infallible>,
        }
    }

    expand_app(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<ErrorTypeProviderComponent> for App {
            type Delegate = UseType<Infallible>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<ErrorTypeProviderComponent, __Context__, __Params__> for App
        where
            UseType<
                Infallible,
            >: IsProviderFor<ErrorTypeProviderComponent, __Context__, __Params__>,
        {}
        ")
    }
}

#[cgp_computer]
pub fn field_to_string<Tag, Value>(Field { value, .. }: Field<Tag, Value>) -> String
where
    Value: Display,
{
    value.to_string()
}

#[cgp_computer]
pub fn value_to_string_ref<Value>(value: &Value) -> String
where
    Value: Display,
{
    value.to_string()
}

#[test]
fn test_dispatch_fields() {
    let context = App;
    let code = PhantomData::<()>;

    assert_eq!(
        MatchWithFieldHandlers::<FieldToString>::compute(&context, code, FooBarBaz::Foo(1)),
        "1"
    );

    assert_eq!(
        MatchWithFieldHandlers::<FieldToString>::compute(
            &context,
            code,
            FooBarBaz::Bar("hello".to_owned())
        ),
        "hello"
    );

    assert_eq!(
        MatchWithFieldHandlers::<FieldToString>::compute(&context, code, FooBarBaz::Baz(true)),
        "true"
    );
}

#[test]
fn test_dispatch_values_ref() {
    let context = App;
    let code = PhantomData::<()>;

    assert_eq!(
        MatchWithValueHandlersRef::<ValueToStringRef>::compute(&context, code, &FooBarBaz::Foo(1)),
        "1"
    );

    assert_eq!(
        MatchWithValueHandlersRef::<ValueToStringRef>::compute_ref(
            &context,
            code,
            &FooBarBaz::Foo(1)
        ),
        "1"
    );

    assert_eq!(
        MatchWithValueHandlersRef::<ValueToStringRef>::compute(
            &context,
            code,
            &FooBarBaz::Bar("hello".to_owned())
        ),
        "hello"
    );

    assert_eq!(
        MatchWithValueHandlersRef::<ValueToStringRef>::compute(
            &context,
            code,
            &FooBarBaz::Baz(true)
        ),
        "true"
    );
}

snapshot_cgp_new_provider! {
    #[cgp_new_provider]
    impl<Context, Code, Value> Computer<Context, Code, &Value> for ValueToString
    where
        Value: Display,
    {
        type Output = String;

        fn compute(_context: &Context, _code: PhantomData<Code>, input: &Value) -> Self::Output {
            input.to_string()
        }
    }

    expand_value_to_string(output) {
        insta::assert_snapshot!(output, @"
        impl<Context, Code, Value> Computer<Context, Code, &Value> for ValueToString
        where
            Value: Display,
        {
            type Output = String;
            fn compute(
                _context: &Context,
                _code: PhantomData<Code>,
                input: &Value,
            ) -> Self::Output {
                input.to_string()
            }
        }
        impl<Context, Code, Value> IsProviderFor<ComputerComponent, Context, (Code, &Value)>
        for ValueToString
        where
            Value: Display,
        {}
        pub struct ValueToString;
        ")
    }
}

#[test]
fn test_dispatch_fields_ref() {
    let context = App;
    let code = PhantomData::<()>;

    assert_eq!(
        MatchWithValueHandlersRef::<ValueToString>::compute(&context, code, &FooBarBaz::Foo(1)),
        "1"
    );

    assert_eq!(
        MatchWithValueHandlersRef::<ValueToString>::compute(
            &context,
            code,
            &FooBarBaz::Bar("hello".to_owned())
        ),
        "hello"
    );

    assert_eq!(
        MatchWithValueHandlersRef::<ValueToString>::compute(&context, code, &FooBarBaz::Baz(true)),
        "true"
    );
}

#[test]
fn test_async_dispatch_fields() {
    let context = App;
    let code = PhantomData::<()>;

    assert_eq!(
        block_on(MatchWithFieldHandlers::<FieldToString>::compute_async(
            &context,
            code,
            FooBarBaz::Foo(1)
        )),
        "1"
    );

    assert_eq!(
        block_on(MatchWithFieldHandlers::<FieldToString>::compute_async(
            &context,
            code,
            FooBarBaz::Bar("hello".to_owned())
        )),
        "hello"
    );

    assert_eq!(
        block_on(MatchWithFieldHandlers::<FieldToString>::compute_async(
            &context,
            code,
            FooBarBaz::Baz(true)
        )),
        "true"
    );
}

#[cgp_computer]
pub fn show_foo_bar(input: FooBar) -> String {
    format!("FooBar::{input:?}")
}

#[cgp_computer]
pub fn show_baz(input: bool) -> String {
    format!("Baz({input:?})")
}

type Computers = Product![
    ExtractFieldAndHandle<Symbol!("Baz"), HandleFieldValue<ShowBaz>>,
    DowncastAndHandle<FooBar, ShowFooBar>,
];

type Handlers = Product![
    PromoteAsync<ExtractFieldAndHandle<Symbol!("Baz"), HandleFieldValue<ShowBaz>>>,
    PromoteAsync<DowncastAndHandle<FooBar, ShowFooBar>>,
];

#[test]
fn test_dispatch_computers() {
    let context = App;
    let code = PhantomData::<()>;

    assert_eq!(
        MatchWithHandlers::<Computers>::compute(&context, code, FooBarBaz::Foo(1)),
        "FooBar::Foo(1)"
    );

    assert_eq!(
        MatchWithHandlers::<Computers>::compute(&context, code, FooBarBaz::Bar("hello".to_owned())),
        "FooBar::Bar(\"hello\")"
    );

    assert_eq!(
        MatchWithHandlers::<Computers>::compute(&context, code, FooBarBaz::Baz(true)),
        "Baz(true)"
    );
}

#[test]
fn test_dispatch_handlers() {
    let context = App;
    let code = PhantomData::<()>;

    assert_eq!(
        block_on(MatchWithHandlers::<Handlers>::compute_async(
            &context,
            code,
            FooBarBaz::Foo(1)
        )),
        "FooBar::Foo(1)"
    );

    assert_eq!(
        block_on(MatchWithHandlers::<Handlers>::compute_async(
            &context,
            code,
            FooBarBaz::Bar("hello".to_owned())
        )),
        "FooBar::Bar(\"hello\")"
    );

    assert_eq!(
        block_on(MatchWithHandlers::<Handlers>::compute_async(
            &context,
            code,
            FooBarBaz::Baz(true)
        )),
        "Baz(true)"
    );
}
