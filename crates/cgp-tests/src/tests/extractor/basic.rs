use core::convert::Infallible;
use core::fmt::{Debug, Display};
use core::marker::PhantomData;

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::core::field::CanExtractInto;
use cgp::extra::dispatch::{
    DispatchFields, DispatchHandlers, ExtractAndHandle, ExtractFieldAndHandle,
};
use cgp::extra::handler::{Computer, ComputerComponent, Handler, Promote};
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
fn test_extract_from() {
    assert_eq!(
        FooBarBaz::Foo(1)
            .extractor()
            .extract_into(PhantomData::<FooBar>)
            .ok(),
        Some(FooBar::Foo(1))
    );
    assert_eq!(
        FooBarBaz::Bar("hello".to_owned())
            .extractor()
            .extract_into(PhantomData::<FooBar>)
            .ok(),
        Some(FooBar::Bar("hello".to_owned()))
    );
    assert_eq!(
        FooBarBaz::Baz(true)
            .extractor()
            .extract_into(PhantomData::<FooBar>)
            .ok(),
        None
    );

    {
        let remainder = FooBarBaz::Baz(true)
            .extractor()
            .extract_into(PhantomData::<FooBar>)
            .unwrap_err();
        assert_eq!(
            remainder.extract_into(PhantomData::<Baz>).ok(),
            Some(Baz::Baz(true))
        );
    }

    assert_eq!(
        FooBarBaz::Foo(1)
            .extractor()
            .extract_into(PhantomData::<BazBarFoo>)
            .ok(),
        Some(BazBarFoo::Foo(1))
    );

    assert_eq!(
        FooBarBaz::Bar("hello".to_owned())
            .extractor()
            .extract_into(PhantomData::<BazBarFoo>)
            .ok(),
        Some(BazBarFoo::Bar("hello".to_owned()))
    );

    assert_eq!(
        FooBarBaz::Baz(true)
            .extractor()
            .extract_into(PhantomData::<BazBarFoo>)
            .ok(),
        Some(BazBarFoo::Baz(true))
    );
}

#[test]
fn test_dispatch_fields() {
    #[cgp_new_provider]
    impl<Context, Code, Tag, Value> Computer<Context, Code, Field<Tag, Value>> for FieldToString
    where
        Value: Display,
    {
        type Output = String;

        fn compute(
            _context: &Context,
            _tag: PhantomData<Code>,
            input: Field<Tag, Value>,
        ) -> String {
            input.value.to_string()
        }
    }

    let context = ();
    let code = PhantomData::<()>;

    assert_eq!(
        DispatchFields::<FieldToString>::compute(&context, code, FooBarBaz::Foo(1)),
        "1"
    );

    assert_eq!(
        DispatchFields::<FieldToString>::compute(
            &context,
            code,
            FooBarBaz::Bar("hello".to_owned())
        ),
        "hello"
    );

    assert_eq!(
        DispatchFields::<FieldToString>::compute(&context, code, FooBarBaz::Baz(true)),
        "true"
    );
}

#[cgp_context]
pub struct App;

delegate_components! {
    AppComponents {
        ErrorTypeProviderComponent: UseType<Infallible>,
    }
}

#[cgp_new_provider]
impl<Context, Code> Computer<Context, Code, FooBar> for Show {
    type Output = String;

    fn compute(_context: &Context, _tag: PhantomData<Code>, input: FooBar) -> String {
        format!("FooBar::{:?}", input)
    }
}

#[cgp_provider]
impl<Context, Code> Computer<Context, Code, Field<symbol!("Baz"), bool>> for Show {
    type Output = String;

    fn compute(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Field<symbol!("Baz"), bool>,
    ) -> String {
        format!("Baz({:?})", input)
    }
}

type Computers =
    Product![ExtractFieldAndHandle<symbol!("Baz"), Show>, ExtractAndHandle<FooBar, Show>];

type Handlers = Product![
    Promote<ExtractFieldAndHandle<symbol!("Baz"), Show>>,
    Promote<ExtractAndHandle<FooBar, Show>>
];

#[test]
fn test_dispatch_computers() {
    let context = App;
    let code = PhantomData::<()>;

    assert_eq!(
        DispatchHandlers::<Computers>::compute(&context, code, FooBarBaz::Foo(1)),
        "FooBar::Foo(1)"
    );

    assert_eq!(
        DispatchHandlers::<Computers>::compute(&context, code, FooBarBaz::Bar("hello".to_owned())),
        "FooBar::Bar(\"hello\")"
    );

    assert_eq!(
        DispatchHandlers::<Computers>::compute(&context, code, FooBarBaz::Baz(true)),
        "Baz(true)"
    );
}

#[test]
fn test_dispatch_handlers() {
    let context = App;
    let code = PhantomData::<()>;

    assert_eq!(
        block_on(DispatchHandlers::<Handlers>::handle(
            &context,
            code,
            FooBarBaz::Foo(1)
        ))
        .unwrap(),
        "FooBar::Foo(1)"
    );

    assert_eq!(
        block_on(DispatchHandlers::<Handlers>::handle(
            &context,
            code,
            FooBarBaz::Bar("hello".to_owned())
        ))
        .unwrap(),
        "FooBar::Bar(\"hello\")"
    );

    assert_eq!(
        block_on(DispatchHandlers::<Handlers>::handle(
            &context,
            code,
            FooBarBaz::Baz(true)
        ))
        .unwrap(),
        "Baz(true)"
    );
}
