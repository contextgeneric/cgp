//! `#[cgp_producer]` with an explicit name, an omitted return type, and a
//! `Result` return type.
//!
//! An explicit name replaces the PascalCase default. An omitted return type is
//! `()`. A producer has one expansion whatever it returns: a `Result` output is
//! a plain value, so `try_compute` wraps it in `Ok` instead of propagating its
//! error.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_producer.md and
//! cgp-knowledge-base-fork/cgp/reference/macros/cgp_producer.md.

use cgp_fork::core::error::ErrorTypeProviderComponent;
use cgp_fork::extra::handler::{Producer, TryComputer};
use cgp_fork::prelude::*;
use cgp_fork_macro_test_util::snapshot_cgp_producer;

snapshot_cgp_producer! {
    #[cgp_producer(TheAnswer)]
    fn magic_number() -> u64 {
        42
    }

    expand_named_producer(output) {
        insta::assert_snapshot!(output, @"
        fn magic_number() -> u64 {
            42
        }
        impl<__Context__, __Code__> Producer<__Context__, __Code__> for TheAnswer {
            type Output = u64;
            fn produce(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
            ) -> Self::Output {
                magic_number()
            }
        }
        impl<__Context__, __Code__> IsProviderFor<ProducerComponent, __Context__, (__Code__)>
        for TheAnswer {}
        pub struct TheAnswer;
        impl DelegateComponent<ComputerComponent> for TheAnswer {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<ComputerComponent, __Context__, __Params__> for TheAnswer
        where
            PromoteProducer<Self>: IsProviderFor<ComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<ComputerRefComponent> for TheAnswer {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<ComputerRefComponent, __Context__, __Params__> for TheAnswer
        where
            PromoteProducer<Self>: IsProviderFor<ComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerComponent> for TheAnswer {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerComponent, __Context__, __Params__> for TheAnswer
        where
            PromoteProducer<Self>: IsProviderFor<TryComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerRefComponent> for TheAnswer {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerRefComponent, __Context__, __Params__> for TheAnswer
        where
            PromoteProducer<
                Self,
            >: IsProviderFor<TryComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerComponent> for TheAnswer {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerComponent, __Context__, __Params__> for TheAnswer
        where
            PromoteProducer<
                Self,
            >: IsProviderFor<AsyncComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerRefComponent> for TheAnswer {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for TheAnswer
        where
            PromoteProducer<
                Self,
            >: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for TheAnswer {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for TheAnswer
        where
            PromoteProducer<Self>: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for TheAnswer {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for TheAnswer
        where
            PromoteProducer<Self>: IsProviderFor<HandlerRefComponent, __Context__, __Params__>,
        {}
        ")
    }
}

#[cgp_producer]
fn nothing() {}

#[cgp_producer]
fn failing() -> Result<u64, String> {
    Err("nope".to_owned())
}

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent:
            UseType<String>,
    }
}

#[test]
fn test_producer_output() {
    let code = PhantomData::<()>;

    assert_eq!(TheAnswer::produce(&App, code), 42);

    Nothing::produce(&App, code);

    assert_eq!(
        Failing::try_compute(&App, code, ()),
        Ok(Err("nope".to_owned())),
    );
}
