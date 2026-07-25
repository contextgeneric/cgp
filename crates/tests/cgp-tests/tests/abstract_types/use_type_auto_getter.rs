//! `#[use_type]` importing an abstract type into a `#[cgp_auto_getter]` trait.
//!
//! A getter macro routes its companion attributes through the same
//! `CgpComponentAttributes` collector as `#[cgp_component]`, so `#[use_type]`
//! works on it too: `#[use_type(HasScalarType.Scalar)]` adds `HasScalarType` as a
//! supertrait of the generated getter trait and rewrites the bare `Scalar` return
//! type to `<Self as HasScalarType>::Scalar`. The derived blanket impl then reads
//! a `base_value` field whose `HasField` value type is that qualified associated
//! type, so `App` supplies both the concrete scalar (via `UseType<f64>`) and the
//! field. This pins that `#[use_type]` is supported on `#[cgp_auto_getter]`, not
//! only on the three implementation macros.
//!
//! See cgp-knowledge-base/cgp/reference/attributes/use_type.md and
//! cgp-knowledge-base/cgp/concepts/abstract-types.md.

use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

// Expands, after the `#[use_type]` rewrite, to roughly:
//   pub trait HasBaseValue: HasScalarType {
//       fn base_value(&self) -> &<Self as HasScalarType>::Scalar;
//   }
// with a blanket impl reading a `base_value` field of that type.
#[cgp_auto_getter]
#[use_type(HasScalarType.Scalar)]
pub trait HasBaseValue {
    fn base_value(&self) -> &Scalar;
}

#[derive(HasField)]
pub struct App {
    pub base_value: f64,
}

delegate_and_check_components! {
    App {
        ScalarTypeProviderComponent: UseType<f64>,
    }
}

#[test]
fn test_base_value() {
    let app = App { base_value: 42.0 };
    assert_eq!(*app.base_value(), 42.0);
}
