//! `#[cgp_computer]` across parameter counts and patterns.
//!
//! The input type is the parameter types written inside parentheses, so no
//! parameters give `()`, one parameter gives the bare type (`(u64)` is `u64`),
//! and two or more give a tuple. Parameter patterns are discarded: each input is
//! rebound positionally as `arg_i`, so a `mut` binding or a destructuring pattern
//! still compiles, and the function itself keeps its own patterns.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_computer.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_computer.md.

use cgp::extra::handler::Computer;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_computer;

snapshot_cgp_computer! {
    #[cgp_computer]
    fn zero() -> u64 {
        0
    }

    expand_no_parameters(output) {
        insta::assert_snapshot!(output, @"
        fn zero() -> u64 {
            0
        }
        #[cgp_new_provider]
        impl<__Context__, __Code__> Computer<__Context__, __Code__, ()> for Zero {
            type Output = u64;
            fn compute(
                _context: &__Context__,
                _code: PhantomData<__Code__>,
                (): (),
            ) -> Self::Output {
                zero()
            }
        }
        delegate_components! {
            Zero { [ComputerRefComponent, TryComputerComponent, TryComputerRefComponent,
            AsyncComputerComponent, AsyncComputerRefComponent, HandlerComponent,
            HandlerRefComponent,] -> PromoteComputer < Self >, }
        }
        ")
    }
}

snapshot_cgp_computer! {
    #[cgp_computer]
    fn increment(value: u64) -> u64 {
        value + 1
    }

    expand_one_parameter(output) {
        insta::assert_snapshot!(output, @"
        fn increment(value: u64) -> u64 {
            value + 1
        }
        #[cgp_new_provider]
        impl<__Context__, __Code__> Computer<__Context__, __Code__, (u64)> for Increment {
            type Output = u64;
            fn compute(
                _context: &__Context__,
                _code: PhantomData<__Code__>,
                (arg_0): (u64),
            ) -> Self::Output {
                increment(arg_0)
            }
        }
        delegate_components! {
            Increment { [ComputerRefComponent, TryComputerComponent, TryComputerRefComponent,
            AsyncComputerComponent, AsyncComputerRefComponent, HandlerComponent,
            HandlerRefComponent,] -> PromoteComputer < Self >, }
        }
        ")
    }
}

#[cgp_computer]
fn decrement(mut value: u64) -> u64 {
    value -= 1;
    value
}

#[cgp_computer]
fn sum_pair((a, b): (u64, u64), scale: u64) -> u64 {
    (a + b) * scale
}

pub struct App;

#[test]
fn test_computer_arity() {
    let code = PhantomData::<()>;

    assert_eq!(Zero::compute(&App, code, ()), 0);
    assert_eq!(Increment::compute(&App, code, 1), 2);
    assert_eq!(Decrement::compute(&App, code, 2), 1);
    assert_eq!(SumPair::compute(&App, code, ((1, 2), 3)), 9);
}
