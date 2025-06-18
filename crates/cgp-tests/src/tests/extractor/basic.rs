use std::marker::PhantomData;

use cgp::prelude::*;

#[derive(ExtractField)]
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

#[test]
fn test_basic_extractor() {
    assert_eq!(context_to_string(Context::Foo(1)), "1");
    assert_eq!(context_to_string(Context::Bar("hello".to_owned())), "hello");
    assert_eq!(context_to_string(Context::Baz(true)), "true");
}
