//! `PipeHandlers` chaining `Handler` stages, including a `Computer` lifted into
//! a `Handler` with the `Promote*` adapters.
//!
//! A pipeline built at the `HandlerComponent` level runs async, fallible stages
//! in order. A middle stage here is a plain `Computer` (`Add`) that is lifted
//! into the pipeline with `PromoteAsync<Promote<...>>`: `Promote` turns a
//! `Computer` into a `TryComputer`/`Handler`-shaped step and `PromoteAsync`
//! makes it async, so a simpler synchronous handler composes alongside async
//! ones. The context uses `Infallible` as its error type since no stage fails.
//!
//! The providers here are incidental scaffolding written with the plain
//! `#[cgp_new_provider]`, and the wiring / check use plain `delegate_components!`
//! / `check_components!` (those macros are owned by other concept targets).
//!
//! See docs/reference/providers/handler_combinators.md and
//! docs/reference/components/handler.md.

use core::convert::Infallible;
use core::marker::PhantomData;

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::extra::handler::{
    CanHandle, Computer, Handler, HandlerComponent, PipeHandlers, Promote, PromoteAsync,
};
use cgp::prelude::*;
use futures::executor::block_on;

#[cgp_new_provider]
impl<Context, Tag, Field> Handler<Context, Tag, u64> for Multiply<Field>
where
    Context: HasErrorType + HasField<Field, Value = u64>,
{
    type Output = u64;

    async fn handle(
        context: &Context,
        _tag: PhantomData<Tag>,
        input: u64,
    ) -> Result<Self::Output, Context::Error> {
        Ok(input * context.get_field(PhantomData))
    }
}

#[cgp_new_provider]
impl<Context, Tag, Field> Computer<Context, Tag, u64> for Add<Field>
where
    Context: HasField<Field, Value = u64>,
{
    type Output = u64;

    fn compute(context: &Context, _tag: PhantomData<Tag>, input: u64) -> u64 {
        input + context.get_field(PhantomData)
    }
}

#[derive(HasField)]
pub struct MyContext {
    pub foo: u64,
    pub bar: u64,
    pub baz: u64,
}

delegate_components! {
    MyContext {
        ErrorTypeProviderComponent: UseType<Infallible>,
        HandlerComponent:
            PipeHandlers<
                Product! [
                    Multiply<Symbol!("foo")>,
                    PromoteAsync<Promote<Add<Symbol!("bar")>>>,
                    Multiply<Symbol!("baz")>,
                ]
            >,
    }
}

check_components! {
    <Tag>
    MyContext {
        HandlerComponent: (Tag, u64),
    }
}

#[test]
pub fn test_pipe_handlers() {
    let context = MyContext {
        foo: 2,
        bar: 3,
        baz: 4,
    };

    let result = block_on(context.handle(PhantomData::<()>, 5)).unwrap();

    assert_eq!(result, ((5 * 2) + 3) * 4);
}
