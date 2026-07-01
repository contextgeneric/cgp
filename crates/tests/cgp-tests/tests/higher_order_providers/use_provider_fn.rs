//! `#[use_provider]` on a `#[cgp_fn]`: the function borrows another provider's
//! behavior instead of depending on the context directly.
//!
//! `#[use_provider(RectangleAreaCalculator: AreaCalculator)]` completes the inner
//! provider's bound (adding the `Self` argument) and moves it into the `where`
//! clause; the body then calls `RectangleAreaCalculator::area(self)`. The
//! component and the inner provider are written plainly (their expansions are
//! owned elsewhere); only the `#[use_provider]` `#[cgp_fn]` is snapshotted here.
//!
//! See docs/reference/attributes/use_provider.md and docs/reference/macros/cgp_fn.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_impl(new RectangleAreaCalculator)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_provider(RectangleAreaCalculator: AreaCalculator)]
    fn rectangle_area(&self) -> f64 {
        RectangleAreaCalculator::area(self)
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        trait RectangleArea {
            fn rectangle_area(&self) -> f64;
        }
        impl<__Context__> RectangleArea for __Context__
        where
            RectangleAreaCalculator: AreaCalculator<Self>,
        {
            fn rectangle_area(&self) -> f64 {
                RectangleAreaCalculator::area(self)
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[test]
fn test_use_provider() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(rectangle.rectangle_area(), 12.0);
}
