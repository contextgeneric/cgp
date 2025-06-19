use core::fmt::{Debug, Display};
use core::marker::PhantomData;

use cgp::core::field::CanExtractInto;
use cgp::extra::dispatch::{DispatchFields, DispatchHandlers, ExtractAndHandle};
use cgp::extra::handler::{Computer, ComputerComponent};
use cgp::prelude::*;

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
fn test_extractor_dispatcher() {
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

    let res = DispatchFields::<FieldToString>::compute(&(), PhantomData::<()>, FooBarBaz::Foo(1));

    assert_eq!(res, "1");
}

#[test]
fn test_dispatch_handlers() {
    #[cgp_new_provider]
    impl<Context, Code, Input> Computer<Context, Code, Input> for ShowDebug
    where
        Input: Debug,
    {
        type Output = String;

        fn compute(_context: &Context, _tag: PhantomData<Code>, input: Input) -> String {
            format!("{:?}", input)
        }
    }

    let res = DispatchHandlers::<
        Product![ExtractAndHandle<FooBar, ShowDebug>, ExtractAndHandle<Baz, ShowDebug>],
    >::compute(&(), PhantomData::<()>, FooBarBaz::Foo(1));

    assert_eq!(res, "Foo(1)");
}
