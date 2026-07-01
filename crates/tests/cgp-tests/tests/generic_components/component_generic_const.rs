//! A `#[cgp_component]` whose `const` item has an abstract type
//! (`const CONSTANT: Self::Unit`), provided by a const-generic provider
//! constrained on that abstract type.
//!
//! This combines a const generic on the provider (`UseConstant<const CONSTANT>`)
//! with an impl-side dependency (`Context: HasUnitType<Unit = u64>`) that ties the
//! const's type to the context's abstract `Unit`. The const-generic provider
//! snapshot is owned here; the `#[cgp_type]`, `delegate_components!`, and
//! `check_components!` wiring is written plainly (owned by the `abstract_types`,
//! `basic_delegation`, and `checking` concepts).
//!
//! See docs/reference/macros/cgp_component.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_provider;

#[cgp_type]
pub trait HasUnitType {
    type Unit;
}

#[cgp_component(ConstantGetter)]
pub trait HasConstant: HasUnitType {
    const CONSTANT: Self::Unit;
}

pub struct UseConstant<const CONSTANT: u64>;

snapshot_cgp_provider! {
    #[cgp_provider]
    impl<Context, const CONSTANT: u64> ConstantGetter<Context> for UseConstant<CONSTANT>
    where
        Context: HasUnitType<Unit = u64>,
    {
        const CONSTANT: u64 = CONSTANT;
    }

    expand_use_constant(output) {
        insta::assert_snapshot!(output, @"
        impl<Context, const CONSTANT: u64> ConstantGetter<Context> for UseConstant<CONSTANT>
        where
            Context: HasUnitType<Unit = u64>,
        {
            const CONSTANT: u64 = CONSTANT;
        }
        impl<Context, const CONSTANT: u64> IsProviderFor<ConstantGetterComponent, Context, ()>
        for UseConstant<CONSTANT>
        where
            Context: HasUnitType<Unit = u64>,
        {}
        ")
    }
}

pub struct MyContext;

delegate_components! {
    MyContext {
        UnitTypeProviderComponent: UseType<u64>,
        ConstantGetterComponent: UseConstant<42>,
    }
}

check_components! {
    MyContext {
        ConstantGetterComponent,
    }
}

#[test]
fn test_component_with_generic_const() {
    assert_eq!(<MyContext as HasConstant>::CONSTANT, 42);
}
