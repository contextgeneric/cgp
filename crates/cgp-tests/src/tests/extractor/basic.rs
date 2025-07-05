use core::convert::Infallible;
use core::fmt::{Debug, Display};
use core::marker::PhantomData;

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::core::field::{CanDowncast, CanDowncastFields, CanUpcast};
use cgp::extra::dispatch::{
    DowncastAndHandle, ExtractFieldAndHandle, MatchWithFieldHandlers, MatchWithHandlers,
    MatchWithValueHandlersRef,
};
use cgp::extra::handler::{
    Computer, ComputerComponent, ComputerRef, ComputerRefComponent, HandleFieldValue, Handler,
    Promote, Promote2,
};
use cgp::prelude::*;
use futures::executor::block_on;

#[derive(Debug, Eq, PartialEq, HasFields, ExtractField, FromVariant)]
pub enum FooBarBaz {
    Foo(u64),
    Bar(String),
    Baz(bool),
}

#[derive(Debug, Eq, PartialEq, HasFields, ExtractField, FromVariant)]
pub enum FooBar {
    Foo(u64),
    Bar(String),
}

#[derive(Debug, Eq, PartialEq, HasFields, ExtractField, FromVariant)]
pub enum Baz {
    Baz(bool),
}

#[derive(Debug, Eq, PartialEq, HasFields, ExtractField, FromVariant)]
pub enum BazBarFoo {
    Baz(bool),
    Bar(String),
    Foo(u64),
}

fn context_to_string(context: FooBarBaz) -> String {
    match context
        .extractor_ref()
        .extract_field(PhantomData::<symbol!("Foo")>)
    {
        Ok(value) => value.to_string(),
        Err(remainder) => match remainder.extract_field(PhantomData::<symbol!("Bar")>) {
            Ok(value) => value.to_string(),
            Err(remainder) => match remainder.extract_field(PhantomData::<symbol!("Baz")>) {
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

#[cgp_context]
pub struct App;

delegate_components! {
    AppComponents {
        ErrorTypeProviderComponent: UseType<Infallible>,
    }
}

#[cgp_computer]
pub fn field_to_string<Tag, Value>(Field { value, .. }: Field<Tag, Value>) -> String
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
        MatchWithFieldHandlers::<FooBarBaz, FieldToString>::compute(
            &context,
            code,
            FooBarBaz::Foo(1)
        ),
        "1"
    );

    assert_eq!(
        MatchWithFieldHandlers::<FooBarBaz, FieldToString>::compute(
            &context,
            code,
            FooBarBaz::Bar("hello".to_owned())
        ),
        "hello"
    );

    assert_eq!(
        MatchWithFieldHandlers::<FooBarBaz, FieldToString>::compute(
            &context,
            code,
            FooBarBaz::Baz(true)
        ),
        "true"
    );
}

#[cgp_new_provider]
impl<Context, Code, Value> ComputerRef<Context, Code, Value> for ValueToString
where
    Value: Display,
{
    type Output = String;

    fn compute_ref(_context: &Context, _code: PhantomData<Code>, input: &Value) -> Self::Output {
        input.to_string()
    }
}

#[test]
fn test_dispatch_fields_ref() {
    let context = App;
    let code = PhantomData::<()>;

    assert_eq!(
        MatchWithValueHandlersRef::<FooBarBaz, ValueToString>::compute_ref(
            &context,
            code,
            &FooBarBaz::Foo(1)
        ),
        "1"
    );

    assert_eq!(
        MatchWithValueHandlersRef::<FooBarBaz, ValueToString>::compute_ref(
            &context,
            code,
            &FooBarBaz::Bar("hello".to_owned())
        ),
        "hello"
    );

    assert_eq!(
        MatchWithValueHandlersRef::<FooBarBaz, ValueToString>::compute_ref(
            &context,
            code,
            &FooBarBaz::Baz(true)
        ),
        "true"
    );
}

#[test]
fn test_async_dispatch_fields() {
    let context = App;
    let code = PhantomData::<()>;

    assert_eq!(
        block_on(MatchWithFieldHandlers::<FooBarBaz, FieldToString>::handle(
            &context,
            code,
            FooBarBaz::Foo(1)
        ))
        .unwrap(),
        "1"
    );

    assert_eq!(
        block_on(MatchWithFieldHandlers::<FooBarBaz, FieldToString>::handle(
            &context,
            code,
            FooBarBaz::Bar("hello".to_owned())
        ))
        .unwrap(),
        "hello"
    );

    assert_eq!(
        block_on(MatchWithFieldHandlers::<FooBarBaz, FieldToString>::handle(
            &context,
            code,
            FooBarBaz::Baz(true)
        ))
        .unwrap(),
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
    ExtractFieldAndHandle<symbol!("Baz"), HandleFieldValue<ShowBaz>>,
    DowncastAndHandle<FooBar, ShowFooBar>,
];

type Handlers = Product![
    Promote2<ExtractFieldAndHandle<symbol!("Baz"), HandleFieldValue<ShowBaz>>>,
    Promote2<DowncastAndHandle<FooBar, ShowFooBar>>
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
        MatchWithHandlers::<Computers>::try_compute(&context, code, FooBarBaz::Foo(1)),
        Ok("FooBar::Foo(1)".to_owned())
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
        block_on(MatchWithHandlers::<Handlers>::handle(
            &context,
            code,
            FooBarBaz::Foo(1)
        ))
        .unwrap(),
        "FooBar::Foo(1)"
    );

    assert_eq!(
        block_on(MatchWithHandlers::<Handlers>::handle(
            &context,
            code,
            FooBarBaz::Bar("hello".to_owned())
        ))
        .unwrap(),
        "FooBar::Bar(\"hello\")"
    );

    assert_eq!(
        block_on(MatchWithHandlers::<Handlers>::handle(
            &context,
            code,
            FooBarBaz::Baz(true)
        ))
        .unwrap(),
        "Baz(true)"
    );
}
