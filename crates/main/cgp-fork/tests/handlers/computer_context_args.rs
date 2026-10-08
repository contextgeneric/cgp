//! `#[cgp_computer]` reads `#[field]` and `#[implicit]` arguments from the context.
//!
//! Those arguments leave the handler input. The provider calls the original
//! function with the field values, and any remaining parameters stay the input
//! the caller passes to `compute`. The original function is unchanged apart from
//! the attributes, so it can still be called directly.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_computer.md and
//! cgp-knowledge-base-fork/cgp/reference/attributes/implicit.md.

use cgp_fork::prelude::*;
use futures::executor::block_on;

#[derive(HasField)]
struct Rectangle {
    width: f64,
    height: f64,
}

#[cgp_computer]
fn rectangle_area(#[field] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}

#[cgp_computer]
fn scaled_width(#[field] width: f64, factor: f64) -> f64 {
    width * factor
}

#[cgp_computer]
async fn async_area(#[field] width: f64, #[field] height: f64) -> f64 {
    width * height
}

#[test]
fn test_context_arguments() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    let code = PhantomData::<()>;

    assert_eq!(rectangle_area(3.0, 4.0), 12.0);
    assert_eq!(RectangleArea::compute(&rectangle, code, ()), 12.0);
    assert_eq!(ScaledWidth::compute(&rectangle, code, 2.0), 6.0);
    assert_eq!(
        block_on(AsyncArea::compute_async(&rectangle, code, ())),
        12.0
    );
}
