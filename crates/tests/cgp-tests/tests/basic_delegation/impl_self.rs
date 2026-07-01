//! `#[cgp_impl(Self)]` implements the consumer trait directly on the context
//! rather than on a separate provider struct.
//!
//! With `Self` as the provider name and `#[use_provider]` to borrow another
//! provider's behavior, a context can implement its own consumer trait by
//! forwarding to a reusable provider. This is the direct-impl end of the
//! modularity spectrum, where no wiring table is involved.
//!
//! See docs/reference/macros/cgp_impl.md and
//! docs/reference/attributes/use_provider.md.

use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_impl(new RectangleArea)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

// `impl CanCalculateArea for Rectangle`, written in provider style: `Self` is the
// context, and `RectangleArea` supplies the actual computation.
#[cgp_impl(Self)]
#[use_provider(RectangleArea: AreaCalculator)]
impl CanCalculateArea for Rectangle {
    fn area(&self) -> f64 {
        RectangleArea::area(self)
    }
}

#[test]
fn test_impl_self() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(rectangle.area(), 12.0);
}
