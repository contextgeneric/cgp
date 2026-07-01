//! A `#[cgp_component]` carrying a `const` item, provided by a const-generic
//! provider `UseConstant<const CONSTANT: u64>`.
//!
//! The const generic on the provider struct flows through the generated provider
//! trait impl and its `IsProviderFor` impl unchanged; wiring the context to
//! `UseConstant<42>` fixes the constant. This file owns the const-generic
//! provider snapshot; the `delegate_and_check_components!` wiring is written
//! plainly (its expansion is owned by the `basic_delegation` / `checking`
//! concepts).
//!
//! See docs/reference/macros/cgp_component.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_provider;

#[cgp_component(ConstantGetter)]
pub trait HasConstant {
    const CONSTANT: u64;
}

pub struct UseConstant<const CONSTANT: u64>;

snapshot_cgp_provider! {
    #[cgp_provider]
    impl<Context, const CONSTANT: u64> ConstantGetter<Context> for UseConstant<CONSTANT> {
        const CONSTANT: u64 = CONSTANT;
    }

    expand_use_constant(output) {
        insta::assert_snapshot!(output, @"
        impl<Context, const CONSTANT: u64> ConstantGetter<Context> for UseConstant<CONSTANT> {
            const CONSTANT: u64 = CONSTANT;
        }
        impl<Context, const CONSTANT: u64> IsProviderFor<ConstantGetterComponent, Context, ()>
        for UseConstant<CONSTANT> {}
        ")
    }
}

pub struct MyContext;

delegate_and_check_components! {
    MyContext {
        ConstantGetterComponent: UseConstant<42>,
    }
}

#[test]
fn test_component_with_const() {
    assert_eq!(<MyContext as HasConstant>::CONSTANT, 42);
}
