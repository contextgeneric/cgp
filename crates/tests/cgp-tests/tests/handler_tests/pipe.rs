mod pipe_computers {
    use core::marker::PhantomData;

    use cgp::extra::handler::{CanCompute, Computer, ComputerComponent, PipeHandlers};
    use cgp::prelude::*;
    use cgp_macro_test_util::{
        snapshot_cgp_new_provider, snapshot_check_components, snapshot_delegate_components,
    };

    snapshot_cgp_new_provider! {
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

        expand_multiply(output) {
            insta::assert_snapshot!(output, @"
            impl<Context, Tag, Field> Computer<Context, Tag, u64> for Multiply<Field>
            where
                Context: HasField<Field, Value = u64>,
            {
                type Output = u64;
                fn compute(context: &Context, _tag: PhantomData<Tag>, input: u64) -> u64 {
                    input * context.get_field(PhantomData)
                }
            }
            impl<Context, Tag, Field> IsProviderFor<ComputerComponent, Context, (Tag, u64)>
            for Multiply<Field>
            where
                Context: HasField<Field, Value = u64>,
            {}
            pub struct Multiply<Field>(pub ::core::marker::PhantomData<(Field)>);
            ")
        }
    }

    snapshot_cgp_new_provider! {
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

        expand_add(output) {
            insta::assert_snapshot!(output, @"
            impl<Context, Tag, Field> Computer<Context, Tag, u64> for Add<Field>
            where
                Context: HasField<Field, Value = u64>,
            {
                type Output = u64;
                fn compute(context: &Context, _tag: PhantomData<Tag>, input: u64) -> u64 {
                    input + context.get_field(PhantomData)
                }
            }
            impl<Context, Tag, Field> IsProviderFor<ComputerComponent, Context, (Tag, u64)>
            for Add<Field>
            where
                Context: HasField<Field, Value = u64>,
            {}
            pub struct Add<Field>(pub ::core::marker::PhantomData<(Field)>);
            ")
        }
    }

    #[derive(HasField)]
    pub struct MyContext {
        pub foo: u64,
        pub bar: u64,
        pub baz: u64,
    }

    snapshot_delegate_components! {
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

        expand_pipe_computers(output) {
            insta::assert_snapshot!(output, @r#"
            impl DelegateComponent<ComputerComponent> for MyContext {
                type Delegate = PipeHandlers<
                    Product![
                        Multiply < Symbol!("foo") >, Add < Symbol!("bar") >, Multiply <
                        Symbol!("baz") >,
                    ],
                >;
            }
            impl<__Context__, __Params__> IsProviderFor<ComputerComponent, __Context__, __Params__>
            for MyContext
            where
                PipeHandlers<
                    Product![
                        Multiply < Symbol!("foo") >, Add < Symbol!("bar") >, Multiply <
                        Symbol!("baz") >,
                    ],
                >: IsProviderFor<ComputerComponent, __Context__, __Params__>,
            {}
            "#)
        }
    }

    snapshot_check_components! {
        check_components! {
            <Tag>
            MyContext {
                ComputerComponent: (Tag, u64),
            }
        }

        expand_check_pipe_computers(output) {
            insta::assert_snapshot!(output, @"
            trait __CheckMyContext<
                __Component__,
                __Params__: ?Sized,
            >: CanUseComponent<__Component__, __Params__> {}
            impl<Tag> __CheckMyContext<ComputerComponent, (Tag, u64)> for MyContext {}
            ")
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
}

mod pipe_handlers {
    use core::convert::Infallible;
    use core::marker::PhantomData;

    use cgp::core::error::ErrorTypeProviderComponent;
    use cgp::extra::handler::{
        CanHandle, Computer, Handler, HandlerComponent, PipeHandlers, Promote, PromoteAsync,
    };
    use cgp::prelude::*;
    use cgp_macro_test_util::{
        snapshot_cgp_new_provider, snapshot_check_components, snapshot_delegate_components,
    };
    use futures::executor::block_on;

    snapshot_cgp_new_provider! {
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

        expand_multiply(output) {
            insta::assert_snapshot!(output, @"
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
            impl<Context, Tag, Field> IsProviderFor<HandlerComponent, Context, (Tag, u64)>
            for Multiply<Field>
            where
                Context: HasErrorType + HasField<Field, Value = u64>,
            {}
            pub struct Multiply<Field>(pub ::core::marker::PhantomData<(Field)>);
            ")
        }
    }

    snapshot_cgp_new_provider! {
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

        expand_add(output) {
            insta::assert_snapshot!(output, @"
            impl<Context, Tag, Field> Computer<Context, Tag, u64> for Add<Field>
            where
                Context: HasField<Field, Value = u64>,
            {
                type Output = u64;
                fn compute(context: &Context, _tag: PhantomData<Tag>, input: u64) -> u64 {
                    input + context.get_field(PhantomData)
                }
            }
            impl<Context, Tag, Field> IsProviderFor<ComputerComponent, Context, (Tag, u64)>
            for Add<Field>
            where
                Context: HasField<Field, Value = u64>,
            {}
            pub struct Add<Field>(pub ::core::marker::PhantomData<(Field)>);
            ")
        }
    }

    #[derive(HasField)]
    pub struct MyContext {
        pub foo: u64,
        pub bar: u64,
        pub baz: u64,
    }

    snapshot_delegate_components! {
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

        expand_pipe_handlers(output) {
            insta::assert_snapshot!(output, @r#"
            impl DelegateComponent<ErrorTypeProviderComponent> for MyContext {
                type Delegate = UseType<Infallible>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<ErrorTypeProviderComponent, __Context__, __Params__> for MyContext
            where
                UseType<
                    Infallible,
                >: IsProviderFor<ErrorTypeProviderComponent, __Context__, __Params__>,
            {}
            impl DelegateComponent<HandlerComponent> for MyContext {
                type Delegate = PipeHandlers<
                    Product![
                        Multiply < Symbol!("foo") >, PromoteAsync < Promote < Add < Symbol!("bar")
                        >>>, Multiply < Symbol!("baz") >,
                    ],
                >;
            }
            impl<__Context__, __Params__> IsProviderFor<HandlerComponent, __Context__, __Params__>
            for MyContext
            where
                PipeHandlers<
                    Product![
                        Multiply < Symbol!("foo") >, PromoteAsync < Promote < Add < Symbol!("bar")
                        >>>, Multiply < Symbol!("baz") >,
                    ],
                >: IsProviderFor<HandlerComponent, __Context__, __Params__>,
            {}
            "#)
        }
    }

    snapshot_check_components! {
        check_components! {
            <Tag>
            MyContext {
                HandlerComponent: (Tag, u64),
            }
        }

        expand_check_pipe_handlers(output) {
            insta::assert_snapshot!(output, @"
            trait __CheckMyContext<
                __Component__,
                __Params__: ?Sized,
            >: CanUseComponent<__Component__, __Params__> {}
            impl<Tag> __CheckMyContext<HandlerComponent, (Tag, u64)> for MyContext {}
            ")
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
}
