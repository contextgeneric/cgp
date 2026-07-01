//! Dispatching an extensible-variant input to per-variant handlers. The
//! `#[derive(CgpData)]` enum exposes its variants structurally, and the dispatch
//! combinators (`MatchWithFieldHandlers`, `MatchWithValueHandlersRef`,
//! `MatchWithHandlers` with `ExtractFieldAndHandle`/`DowncastAndHandle`) route
//! each variant to a sub-handler, proving exhaustiveness without a wildcard.
//!
//! The dispatch combinators are owned by the `dispatching`/`handlers` concepts;
//! here the derives are plain scaffolding and the wiring snapshots are trimmed
//! to plain macros. This file exercises the *variant* side end-to-end: matching
//! by field, by value (ref), and via casts.
//!
//! See docs/concepts/extensible-variants.md.

use core::convert::Infallible;
use core::fmt::Display;
use core::marker::PhantomData;

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::extra::dispatch::{
    DowncastAndHandle, ExtractFieldAndHandle, HandleFieldValue, MatchWithFieldHandlers,
    MatchWithHandlers, MatchWithValueHandlersRef,
};
use cgp::extra::handler::{Computer, ComputerComponent, ComputerRef, PromoteAsync};
use cgp::prelude::*;
use futures::executor::block_on;

#[derive(Debug, Eq, PartialEq, CgpData)]
pub enum FooBarBaz {
    Foo(u64),
    Bar(String),
    Baz(bool),
}

#[derive(Debug, Eq, PartialEq, CgpData)]
pub enum FooBar {
    Foo(u64),
    Bar(String),
}

pub struct App;

delegate_components! {
    App {
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
