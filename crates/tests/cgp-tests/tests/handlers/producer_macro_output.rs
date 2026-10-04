//! `#[cgp_producer]` with an explicit name, an omitted return type, and a
//! `Result` return type.
//!
//! An explicit name replaces the PascalCase default. An omitted return type is
//! `()`. A producer has one expansion whatever it returns: a `Result` output is
//! a plain value, so `try_compute` wraps it in `Ok` instead of propagating its
//! error.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_producer.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_producer.md.

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::extra::handler::{Producer, TryComputer};
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_producer;

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
        #[cgp_new_provider]
        impl<__Context__, __Code__> Producer<__Context__, __Code__> for TheAnswer {
            type Output = u64;
            fn produce(_context: &__Context__, _code: PhantomData<__Code__>) -> Self::Output {
                magic_number()
            }
        }
        delegate_components! {
            TheAnswer { [ComputerComponent, ComputerRefComponent, TryComputerComponent,
            TryComputerRefComponent, AsyncComputerComponent, AsyncComputerRefComponent,
            HandlerComponent, HandlerRefComponent,] : PromoteProducer < Self >, }
        }
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
