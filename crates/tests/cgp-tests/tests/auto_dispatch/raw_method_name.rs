//! A dispatch trait whose method has a raw identifier as its name, `r#type`. The
//! generated computer and helper are named from the unrawed name (`ComputeType` and
//! `__compute_type__`), since `r#` cannot appear inside a longer identifier.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_dispatch;

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

pub struct Circle;

pub struct Square;

snapshot_cgp_auto_dispatch! {
    #[cgp_auto_dispatch]
    pub trait HasKind {
        fn r#type(&self) -> &'static str;
    }

    expand_raw_method_name(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasKind {
            fn r#type(&self) -> &'static str;
        }
        impl<__Variants__> HasKind for __Variants__
        where
            MatchWithValueHandlersRef<
                ComputeType,
            >: for<'__a__> Computer<(), (), &'__a__ __Variants__, Output = &'static str>,
            __Variants__: HasExtractor,
        {
            fn r#type(&self) -> &'static str {
                <MatchWithValueHandlersRef<
                    ComputeType,
                > as Computer<_, _, _>>::compute(&(), ::core::marker::PhantomData::<()>, self)
            }
        }
        #[cgp_computer(ComputeType)]
        fn __compute_type__<'__a__, __Variants__: HasKind>(
            __Variants__: &'__a__ __Variants__,
        ) -> &'static str {
            __Variants__.r#type()
        }
        ")
    }
}

impl HasKind for Circle {
    fn r#type(&self) -> &'static str {
        "circle"
    }
}

impl HasKind for Square {
    fn r#type(&self) -> &'static str {
        "square"
    }
}

#[test]
fn test_raw_method_name_dispatches() {
    assert_eq!(Shape::Circle(Circle).r#type(), "circle");
    assert_eq!(Shape::Square(Square).r#type(), "square");
}
