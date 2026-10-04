//! Table structs that `delegate_components!` declares, carrying a `const`
//! parameter. A `new` target (`<const N: u64> new ConstantTable<N> { … }`) keeps
//! `N` as a const parameter of the declared struct, although the target's type
//! arguments name it without its kind, and a nested inner table
//! (`UseDelegate<new CountTable<const N: usize> { … }>`) accepts a const
//! parameter in its own generic list. The two snapshots pin the declared structs.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/delegate_components.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_components;

#[cgp_component(ConstantGetter)]
pub trait HasConstant {
    const CONSTANT: u64;
}

pub struct UseConstant<const CONSTANT: u64>;

#[cgp_provider]
impl<Context, const CONSTANT: u64> ConstantGetter<Context> for UseConstant<CONSTANT> {
    const CONSTANT: u64 = CONSTANT;
}

// A `new` target generic over a const: the declared struct is `ConstantTable<const N: u64>`.
snapshot_delegate_components! {
    delegate_components! {
        <const N: u64> new ConstantTable<N> {
            ConstantGetterComponent: UseConstant<N>,
        }
    }
    expand_constant_table(output) {
        insta::assert_snapshot!(output, @"
        pub struct ConstantTable<const N: u64>(pub ::core::marker::PhantomData<()>);
        impl<const N: u64> DelegateComponent<ConstantGetterComponent> for ConstantTable<N> {
            type Delegate = UseConstant<N>;
        }
        impl<
            const N: u64,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<ConstantGetterComponent, __Context__, __Params__> for ConstantTable<N>
        where
            UseConstant<N>: IsProviderFor<ConstantGetterComponent, __Context__, __Params__>,
        {}
        ")
    }
}

pub struct App;

delegate_components! {
    App {
        ConstantGetterComponent: ConstantTable<7>,
    }
}

check_components! {
    App {
        ConstantGetterComponent,
    }
}

#[cgp_component(Counter)]
#[derive_delegate(UseDelegate<T>)]
pub trait CanCount<T> {
    fn count(&self, value: &T) -> usize;
}

pub struct TimesN<const N: usize>;

#[cgp_provider]
impl<Context, T, const N: usize> Counter<Context, T> for TimesN<N> {
    fn count(_context: &Context, _value: &T) -> usize {
        N
    }
}

pub struct Wrapper<const N: usize>;

// A nested inner table whose own generic list declares a const parameter.
snapshot_delegate_components! {
    delegate_components! {
        <const N: usize> Wrapper<N> {
            CounterComponent: UseDelegate<new CountTable<const N: usize> {
                String: TimesN<N>,
            }>,
        }
    }
    expand_count_table(output) {
        insta::assert_snapshot!(output, @"
        pub struct CountTable<const N: usize>(pub ::core::marker::PhantomData<()>);
        impl<const N: usize> DelegateComponent<CounterComponent> for Wrapper<N> {
            type Delegate = UseDelegate<CountTable<N>>;
        }
        impl<
            const N: usize,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<CounterComponent, __Context__, __Params__> for Wrapper<N>
        where
            UseDelegate<CountTable<N>>: IsProviderFor<CounterComponent, __Context__, __Params__>,
        {}
        impl<const N: usize> DelegateComponent<String> for CountTable<N> {
            type Delegate = TimesN<N>;
        }
        impl<
            const N: usize,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<String, __Context__, __Params__> for CountTable<N>
        where
            TimesN<N>: IsProviderFor<String, __Context__, __Params__>,
        {}
        ")
    }
}

check_components! {
    <const N: usize> Wrapper<N> {
        CounterComponent: String,
    }
}

#[test]
fn test_const_generic_tables() {
    assert_eq!(<App as HasConstant>::CONSTANT, 7);
    assert_eq!(Wrapper::<3>.count(&"x".to_owned()), 3);
}
