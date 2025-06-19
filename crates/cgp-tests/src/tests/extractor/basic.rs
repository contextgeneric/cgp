use core::fmt::Display;
use core::marker::PhantomData;

use cgp::core::field::CanExtractFrom;
use cgp::extra::handler::{Computer, ComputerComponent, DispatchFields, DispatchHandlers};
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
        FooBar::extract_from(FooBarBaz::Foo(1).extractor()).ok(),
        Some(FooBar::Foo(1))
    );
    assert_eq!(
        FooBar::extract_from(FooBarBaz::Bar("hello".to_owned()).extractor()).ok(),
        Some(FooBar::Bar("hello".to_owned()))
    );
    assert_eq!(
        FooBar::extract_from(FooBarBaz::Baz(true).extractor()).ok(),
        None
    );

    {
        let remainder = FooBar::extract_from(FooBarBaz::Baz(true).extractor()).unwrap_err();
        assert_eq!(Baz::extract_from(remainder).ok(), Some(Baz::Baz(true)));
    }

    assert_eq!(
        BazBarFoo::extract_from(FooBarBaz::Foo(1).extractor()).ok(),
        Some(BazBarFoo::Foo(1))
    );

    assert_eq!(
        BazBarFoo::extract_from(FooBarBaz::Bar("hello".to_owned()).extractor()).ok(),
        Some(BazBarFoo::Bar("hello".to_owned()))
    );

    assert_eq!(
        BazBarFoo::extract_from(FooBarBaz::Baz(true).extractor()).ok(),
        Some(BazBarFoo::Baz(true))
    );
}

#[cgp_new_provider]
impl<Context, Code, Tag, Value> Computer<Context, Code, Field<Tag, Value>> for FieldToString
where
    Value: Display,
{
    type Output = String;

    fn compute(_context: &Context, _tag: PhantomData<Code>, input: Field<Tag, Value>) -> String {
        input.value.to_string()
    }
}

pub trait CheckComputerImpl: Computer<(), (), FooBarBaz> {}
impl CheckComputerImpl for DispatchFields<FieldToString> {}

#[test]
fn test_extractor_dispatcher() {
    // let res = DispatchFields::<FieldToString>::compute(&(), PhantomData::<()>, Context::Foo(1));

    let res = DispatchHandlers::<<FooBarBaz as HasFields>::Fields, FieldToString>::compute(
        &(),
        PhantomData::<()>,
        FooBarBaz::Foo(1).extractor(),
    );

    assert_eq!(res, "1");
}
