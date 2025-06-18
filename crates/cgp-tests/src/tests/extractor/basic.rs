use core::fmt::Display;
use core::marker::PhantomData;

use cgp::extra::handler::{Computer, ComputerComponent, DispatchHandlers};
use cgp::prelude::*;

#[derive(HasFields, ExtractField)]
pub enum Context {
    Foo(u64),
    Bar(String),
    Baz(bool),
}

fn context_to_string(context: Context) -> String {
    match context
        .extractor_ref()
        .extract_field(PhantomData::<symbol!("Foo")>)
    {
        Either::Left(value) => value.to_string(),
        Either::Right(remainder) => match remainder.extract_field(PhantomData::<symbol!("Bar")>) {
            Either::Left(value) => value.to_string(),
            Either::Right(remainder) => {
                match remainder.extract_field(PhantomData::<symbol!("Baz")>) {
                    Either::Left(value) => value.to_string(),
                    Either::Right(remainder) => remainder.finalize_extract(),
                }
            }
        },
    }
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

#[cgp_provider]
impl<Context, Code> Computer<Context, Code, Void> for FieldToString {
    type Output = String;

    fn compute(_context: &Context, _tag: PhantomData<Code>, input: Void) -> String {
        match input {}
    }
}

pub trait CheckComputerImpl: Computer<(), (), <Context as HasExtractor>::Extractor> {}

impl CheckComputerImpl for DispatchHandlers<<Context as HasFields>::Fields, FieldToString> {}

#[test]
fn test_basic_extractor() {
    assert_eq!(context_to_string(Context::Foo(1)), "1");
    assert_eq!(context_to_string(Context::Bar("hello".to_owned())), "hello");
    assert_eq!(context_to_string(Context::Baz(true)), "true");
}

#[test]
fn test_extractor_dispatcher() {
    let res = DispatchHandlers::<<Context as HasFields>::Fields, FieldToString>::compute(
        &(),
        PhantomData::<()>,
        Context::Foo(1).extractor(),
    );

    assert_eq!(res, "1");
}
