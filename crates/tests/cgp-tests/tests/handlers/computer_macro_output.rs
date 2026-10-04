//! How `#[cgp_computer]` reads the return type.
//!
//! An omitted return type is `()`. Only the bare two-argument form
//! `Result<T, E>` selects the fallible promotion bundle; any other spelling,
//! such as the qualified `core::result::Result<T, E>`, is treated as a plain
//! value, so `try_compute` wraps the whole result in `Ok` rather than
//! propagating its error.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_computer.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_computer.md.

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::extra::handler::{Computer, TryComputer};
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_computer;

snapshot_cgp_computer! {
    #[cgp_computer]
    fn ignore(_value: u64) {}

    expand_unit_return(output) {
        insta::assert_snapshot!(output, @"
        fn ignore(_value: u64) {}
        #[cgp_new_provider]
        impl<__Context__, __Code__> Computer<__Context__, __Code__, (u64)> for Ignore {
            type Output = ();
            fn compute(
                _context: &__Context__,
                _code: PhantomData<__Code__>,
                (arg_0): (u64),
            ) -> Self::Output {
                ignore(arg_0)
            }
        }
        delegate_components! {
            Ignore { [ComputerRefComponent, TryComputerComponent, TryComputerRefComponent,
            AsyncComputerComponent, AsyncComputerRefComponent, HandlerComponent,
            HandlerRefComponent,] -> PromoteComputer < Self >, }
        }
        ")
    }
}

snapshot_cgp_computer! {
    #[cgp_computer]
    fn parse_qualified(value: String) -> core::result::Result<u64, String> {
        value.parse().map_err(|_| value)
    }

    expand_qualified_result(output) {
        insta::assert_snapshot!(output, @"
        fn parse_qualified(value: String) -> core::result::Result<u64, String> {
            value.parse().map_err(|_| value)
        }
        #[cgp_new_provider]
        impl<__Context__, __Code__> Computer<__Context__, __Code__, (String)>
        for ParseQualified {
            type Output = core::result::Result<u64, String>;
            fn compute(
                _context: &__Context__,
                _code: PhantomData<__Code__>,
                (arg_0): (String),
            ) -> Self::Output {
                parse_qualified(arg_0)
            }
        }
        delegate_components! {
            ParseQualified { [ComputerRefComponent, TryComputerComponent,
            TryComputerRefComponent, AsyncComputerComponent, AsyncComputerRefComponent,
            HandlerComponent, HandlerRefComponent,] -> PromoteComputer < Self >, }
        }
        ")
    }
}

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent:
            UseType<String>,
    }
}

#[test]
fn test_computer_output() {
    let code = PhantomData::<()>;

    Ignore::compute(&App, code, 1);

    assert_eq!(
        ParseQualified::try_compute(&App, code, "nope".to_owned()),
        Ok(Err("nope".to_owned())),
    );
}
