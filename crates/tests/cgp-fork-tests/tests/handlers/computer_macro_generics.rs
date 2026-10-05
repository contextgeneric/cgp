//! `#[cgp_computer]` on functions with a `where` clause, a lifetime parameter,
//! and a const generic parameter.
//!
//! The function's generics and `where` clause move onto the generated impl, ahead
//! of the reserved `__Context__` and `__Code__` parameters. Each parameter here
//! appears in an input type, which is what keeps it constrained by the impl.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_computer.md and
//! cgp-knowledge-base-fork/cgp/reference/macros/cgp_computer.md.

use core::fmt::Display;

use cgp_fork::extra::handler::Computer;
use cgp_fork::prelude::*;
use cgp_fork_macro_test_util::snapshot_cgp_computer;

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
        impl<T, __Context__, __Code__> Computer<__Context__, __Code__, (T)> for Describe
        where
            T: Display,
        {
            type Output = String;
            fn compute(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0): (T),
            ) -> Self::Output {
                describe(arg_0)
            }
        }
        impl<
            T,
            __Context__,
            __Code__,
        > IsProviderFor<ComputerComponent, __Context__, (__Code__, (T))> for Describe
        where
            T: Display,
        {}
        pub struct Describe;
        impl DelegateComponent<ComputerRefComponent> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<ComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<ComputerRefComponent, __Context__, __Params__> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                ComputerRefComponent,
            >>::Delegate: IsProviderFor<ComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerComponent> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<TryComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerComponent, __Context__, __Params__> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerComponent,
            >>::Delegate: IsProviderFor<TryComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerRefComponent> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<TryComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerRefComponent, __Context__, __Params__> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerRefComponent,
            >>::Delegate: IsProviderFor<TryComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerComponent> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<AsyncComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerComponent, __Context__, __Params__> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerComponent,
            >>::Delegate: IsProviderFor<AsyncComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerRefComponent> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<AsyncComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerRefComponent,
            >>::Delegate: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<HandlerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerComponent,
            >>::Delegate: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<HandlerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<HandlerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for Describe
        where
            PromoteComputer<Self>: DelegateComponent<HandlerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerRefComponent,
            >>::Delegate: IsProviderFor<HandlerRefComponent, __Context__, __Params__>,
        {}
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
        impl<'a, __Context__, __Code__> Computer<__Context__, __Code__, (&'a str)>
        for FirstChar {
            type Output = &'a str;
            fn compute(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0): (&'a str),
            ) -> Self::Output {
                first_char(arg_0)
            }
        }
        impl<
            'a,
            __Context__,
            __Code__,
        > IsProviderFor<ComputerComponent, __Context__, (__Code__, (&'a str))> for FirstChar {}
        pub struct FirstChar;
        impl DelegateComponent<ComputerRefComponent> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<ComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<ComputerRefComponent, __Context__, __Params__> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                ComputerRefComponent,
            >>::Delegate: IsProviderFor<ComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerComponent> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<TryComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerComponent, __Context__, __Params__> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerComponent,
            >>::Delegate: IsProviderFor<TryComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerRefComponent> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<TryComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerRefComponent, __Context__, __Params__> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerRefComponent,
            >>::Delegate: IsProviderFor<TryComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerComponent> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<AsyncComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerComponent, __Context__, __Params__> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerComponent,
            >>::Delegate: IsProviderFor<AsyncComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerRefComponent> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<AsyncComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerRefComponent,
            >>::Delegate: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<HandlerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerComponent,
            >>::Delegate: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<HandlerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<HandlerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for FirstChar
        where
            PromoteComputer<Self>: DelegateComponent<HandlerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerRefComponent,
            >>::Delegate: IsProviderFor<HandlerRefComponent, __Context__, __Params__>,
        {}
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
        impl<const N: usize, __Context__, __Code__> Computer<__Context__, __Code__, ([u64; N])>
        for SumArray {
            type Output = u64;
            fn compute(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0): ([u64; N]),
            ) -> Self::Output {
                sum_array(arg_0)
            }
        }
        impl<
            const N: usize,
            __Context__,
            __Code__,
        > IsProviderFor<ComputerComponent, __Context__, (__Code__, ([u64; N]))> for SumArray {}
        pub struct SumArray;
        impl DelegateComponent<ComputerRefComponent> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<ComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<ComputerRefComponent, __Context__, __Params__> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                ComputerRefComponent,
            >>::Delegate: IsProviderFor<ComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerComponent> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<TryComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerComponent, __Context__, __Params__> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerComponent,
            >>::Delegate: IsProviderFor<TryComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerRefComponent> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<TryComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerRefComponent, __Context__, __Params__> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerRefComponent,
            >>::Delegate: IsProviderFor<TryComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerComponent> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<AsyncComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerComponent, __Context__, __Params__> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerComponent,
            >>::Delegate: IsProviderFor<AsyncComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerRefComponent> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<AsyncComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerRefComponent,
            >>::Delegate: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<HandlerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerComponent,
            >>::Delegate: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<HandlerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<HandlerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for SumArray
        where
            PromoteComputer<Self>: DelegateComponent<HandlerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerRefComponent,
            >>::Delegate: IsProviderFor<HandlerRefComponent, __Context__, __Params__>,
        {}
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
