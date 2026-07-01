//! `PipeHandlers` chaining a series of `Computer` providers left-to-right.
//!
//! Each stage is a `Computer` that reads one context field and folds it into the
//! running value; `PipeHandlers<Product![A, B, C]>` runs them in order so the
//! output of one becomes the input of the next. Wiring `ComputerComponent` to
//! the pipe makes `context.compute(...)` evaluate the whole chain.
//!
//! The providers here are incidental scaffolding written with the plain
//! `#[cgp_new_provider]`, and the wiring / check use plain `delegate_components!`
//! / `check_components!` (those macros are owned by other concept targets).
//!
//! See docs/reference/providers/handler_combinators.md and
//! docs/reference/components/computer.md.

use core::marker::PhantomData;

use cgp::extra::handler::{CanCompute, Computer, ComputerComponent, PipeHandlers};
use cgp::prelude::*;

#[cgp_new_provider]
impl<Context, Tag, Field> Computer<Context, Tag, u64> for Multiply<Field>
where
    Context: HasField<Field, Value = u64>,
{
    type Output = u64;

    fn compute(context: &Context, _tag: PhantomData<Tag>, input: u64) -> u64 {
        input * context.get_field(PhantomData)
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
        ComputerComponent:
            PipeHandlers<
                Product! [
                    Multiply<Symbol!("foo")>,
                    Add<Symbol!("bar")>,
                    Multiply<Symbol!("baz")>,
                ]
            >,
    }
}

check_components! {
    <Tag>
    MyContext {
        ComputerComponent: (Tag, u64),
    }
}

#[test]
pub fn test_pipe_computers() {
    let context = MyContext {
        foo: 2,
        bar: 3,
        baz: 4,
    };

    let result = context.compute(PhantomData::<()>, 5);

    assert_eq!(result, ((5 * 2) + 3) * 4);
}
