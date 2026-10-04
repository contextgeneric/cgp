//! `#[cgp_computer(Name)]`: an explicit provider name replaces the default, which
//! is the function name in PascalCase.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_computer.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_computer.md.

use cgp::extra::handler::Computer;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_computer;

snapshot_cgp_computer! {
    #[cgp_computer(MyAdder)]
    fn add(a: u64, b: u64) -> u64 {
        a + b
    }

    expand_named_computer(output) {
        insta::assert_snapshot!(output, @"
        fn add(a: u64, b: u64) -> u64 {
            a + b
        }
        #[cgp_new_provider]
        impl<__Context__, __Code__> Computer<__Context__, __Code__, (u64, u64)> for MyAdder {
            type Output = u64;
            fn compute(
                _context: &__Context__,
                _code: PhantomData<__Code__>,
                (arg_0, arg_1): (u64, u64),
            ) -> Self::Output {
                add(arg_0, arg_1)
            }
        }
        delegate_components! {
            MyAdder { [ComputerRefComponent, TryComputerComponent, TryComputerRefComponent,
            AsyncComputerComponent, AsyncComputerRefComponent, HandlerComponent,
            HandlerRefComponent,] -> PromoteComputer < Self >, }
        }
        ")
    }
}

pub struct App;

#[test]
fn test_named_computer() {
    assert_eq!(MyAdder::compute(&App, PhantomData::<()>, (1, 2)), 3);
}
