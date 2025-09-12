use core::convert::Infallible;
use core::marker::PhantomData;

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::extra::handler::{
    CanCompute, CanHandle, Computer, ComputerComponent, Handler, HandlerComponent, PipeHandlers,
    Promote, PromoteAsync,
};
use cgp::prelude::*;
use futures::executor::block_on;

#[test]
pub fn test_pipe_computers() {
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

    #[cgp_context]
    #[derive(HasField)]
    pub struct MyContext {
        pub foo: u64,
        pub bar: u64,
        pub baz: u64,
    }

    delegate_components! {
        MyContextComponents {
            ComputerComponent:
                PipeHandlers<
                    Product! [
                        Multiply<symbol!("foo")>,
                        Add<symbol!("bar")>,
                        Multiply<symbol!("baz")>,
                    ]
                >,
        }
    }

    check_components! {
        <Tag>
        CanUseMyContext for MyContext {
            ComputerComponent: (Tag, u64),
        }
    }

    let context = MyContext {
        foo: 2,
        bar: 3,
        baz: 4,
    };

    let result = context.compute(PhantomData::<()>, 5);

    assert_eq!(result, ((5 * 2) + 3) * 4);
}

#[test]
pub fn test_pipe_handlers() {
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

    #[cgp_context]
    #[derive(HasField)]
    pub struct MyContext {
        pub foo: u64,
        pub bar: u64,
        pub baz: u64,
    }

    delegate_components! {
        MyContextComponents {
            ErrorTypeProviderComponent: UseType<Infallible>,
            HandlerComponent:
                PipeHandlers<
                    Product! [
                        Multiply<symbol!("foo")>,
                        PromoteAsync<Promote<Add<symbol!("bar")>>>,
                        Multiply<symbol!("baz")>,
                    ]
                >,
        }
    }

    check_components! {
        <Tag>
        CanUseMyContext for MyContext {
            HandlerComponent: (Tag, u64),
        }
    }

    let context = MyContext {
        foo: 2,
        bar: 3,
        baz: 4,
    };

    let result = block_on(context.handle(PhantomData::<()>, 5)).unwrap();

    assert_eq!(result, ((5 * 2) + 3) * 4);
}
