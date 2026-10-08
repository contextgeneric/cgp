//! `#[derive_promote]` forwards a component method's arguments as the `Computer` input.
//!
//! `scale` takes `factor` in addition to `&self`. The promoted provider passes
//! that argument through to `compute`, while `width` stays a context field on the
//! computer.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_component.md and
//! cgp-knowledge-base-fork/cgp/reference/macros/cgp_computer.md.

use cgp_fork::prelude::*;

#[cgp_component(Scaler)]
#[derive_promote(PromoteScaler)]
pub trait CanScale {
    fn scale(&self, factor: f64) -> f64;
}

#[cgp_computer]
fn scale_width(#[field] width: f64, factor: f64) -> f64 {
    width * factor
}

#[derive(HasField)]
struct Rectangle {
    width: f64,
}

delegate_components! {
    Rectangle {
        ScalerComponent: PromoteScaler<ScaleWidth>,
    }
}

#[test]
fn test_promoted_scale() {
    let rectangle = Rectangle { width: 3.0 };

    assert_eq!(rectangle.scale(2.0), 6.0);
}
