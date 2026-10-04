//! `#[cgp_computer]` on functions with a `where` clause, a lifetime parameter,
//! and a const generic parameter.
//!
//! The function's generics and `where` clause move onto the generated impl, ahead
//! of the reserved `__Context__` and `__Code__` parameters. Each parameter here
//! appears in an input type, which is what keeps it constrained by the impl.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_computer.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_computer.md.

use core::fmt::Display;

use cgp::extra::handler::Computer;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_computer;

snapshot_cgp_computer! {
    #[cgp_computer]
    fn describe<T>(value: T) -> String
    where
        T: Display,
    {
        format!("<{value}>")
    }

    expand_where_clause(output) {
        insta::assert_snapshot!(output, @r#"
        fn describe<T>(value: T) -> String
        where
            T: Display,
        {
            format!("<{value}>")
        }
        #[cgp_new_provider]
        impl<T, __Context__, __Code__> Computer<__Context__, __Code__, (T)> for Describe
        where
            T: Display,
        {
            type Output = String;
            fn compute(
                _context: &__Context__,
                _code: PhantomData<__Code__>,
                (arg_0): (T),
            ) -> Self::Output {
                describe(arg_0)
            }
        }
        delegate_components! {
            Describe { [ComputerRefComponent, TryComputerComponent, TryComputerRefComponent,
            AsyncComputerComponent, AsyncComputerRefComponent, HandlerComponent,
            HandlerRefComponent,] -> PromoteComputer < Self >, }
        }
        "#)
    }
}

snapshot_cgp_computer! {
    #[cgp_computer]
    fn first_char<'a>(value: &'a str) -> &'a str {
        &value[..1]
    }

    expand_lifetime_parameter(output) {
        insta::assert_snapshot!(output, @"
        fn first_char<'a>(value: &'a str) -> &'a str {
            &value[..1]
        }
        #[cgp_new_provider]
        impl<'a, __Context__, __Code__> Computer<__Context__, __Code__, (&'a str)>
        for FirstChar {
            type Output = &'a str;
            fn compute(
                _context: &__Context__,
                _code: PhantomData<__Code__>,
                (arg_0): (&'a str),
            ) -> Self::Output {
                first_char(arg_0)
            }
        }
        delegate_components! {
            FirstChar { [ComputerRefComponent, TryComputerComponent, TryComputerRefComponent,
            AsyncComputerComponent, AsyncComputerRefComponent, HandlerComponent,
            HandlerRefComponent,] -> PromoteComputer < Self >, }
        }
        ")
    }
}

snapshot_cgp_computer! {
    #[cgp_computer]
    fn sum_array<const N: usize>(values: [u64; N]) -> u64 {
        values.iter().sum()
    }

    expand_const_parameter(output) {
        insta::assert_snapshot!(output, @"
        fn sum_array<const N: usize>(values: [u64; N]) -> u64 {
            values.iter().sum()
        }
        #[cgp_new_provider]
        impl<const N: usize, __Context__, __Code__> Computer<__Context__, __Code__, ([u64; N])>
        for SumArray {
            type Output = u64;
            fn compute(
                _context: &__Context__,
                _code: PhantomData<__Code__>,
                (arg_0): ([u64; N]),
            ) -> Self::Output {
                sum_array(arg_0)
            }
        }
        delegate_components! {
            SumArray { [ComputerRefComponent, TryComputerComponent, TryComputerRefComponent,
            AsyncComputerComponent, AsyncComputerRefComponent, HandlerComponent,
            HandlerRefComponent,] -> PromoteComputer < Self >, }
        }
        ")
    }
}

pub struct App;

#[test]
fn test_computer_generics() {
    let code = PhantomData::<()>;

    assert_eq!(Describe::compute(&App, code, 1), "<1>");
    assert_eq!(FirstChar::compute(&App, code, "abc"), "a");
    assert_eq!(SumArray::compute(&App, code, [1, 2, 3]), 6);
}
