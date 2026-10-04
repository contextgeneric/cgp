//! A dispatch trait declared in one module, its payloads and enum in another,
//! and the dispatched method called from a third.
//!
//! The generated per-variant computer and the enum-level blanket impl are emitted
//! beside the trait, so they must stay reachable wherever the trait is: the
//! computer struct is public, and the helper function it calls is private to the
//! trait's module.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

mod traits {
    use cgp::prelude::*;

    #[cgp_auto_dispatch]
    pub trait HasName {
        fn name(&self) -> &'static str;
    }
}

mod shapes {
    use cgp::prelude::*;

    use super::traits::HasName;

    pub struct Circle;
    pub struct Square;

    #[derive(CgpData)]
    pub enum Shape {
        Circle(Circle),
        Square(Square),
    }

    impl HasName for Circle {
        fn name(&self) -> &'static str {
            "circle"
        }
    }

    impl HasName for Square {
        fn name(&self) -> &'static str {
            "square"
        }
    }
}

use shapes::{Shape, Square};
use traits::HasName;

#[test]
fn test_dispatch_across_modules() {
    assert_eq!(Shape::Square(Square).name(), "square");
}
