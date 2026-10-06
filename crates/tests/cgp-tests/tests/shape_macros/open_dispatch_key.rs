//! A shape can key an `open` dispatch entry, so a component dispatched per type
//! can choose a provider by the structure of a record. The brace form follows
//! the `@Component.` prefix directly, where the path grammar also accepts its
//! own `{ … }` grouping, so this pins that the macro is read as one type.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/struct.md.

use cgp::prelude::*;

#[cgp_component(ShapeDescriber)]
pub trait CanDescribeShape<Shape> {
    fn describe_shape(&self) -> &'static str;
}

#[cgp_impl(new DescribePoint)]
impl<Shape> ShapeDescriber<Shape> {
    fn describe_shape(&self) -> &'static str {
        "point"
    }
}

#[cgp_impl(new DescribePair)]
impl<Shape> ShapeDescriber<Shape> {
    fn describe_shape(&self) -> &'static str {
        "pair"
    }
}

pub struct App;

delegate_components! {
    App {
        open ShapeDescriberComponent;

        @ShapeDescriberComponent.Struct! { x: f64, y: f64 }: DescribePoint,
        @ShapeDescriberComponent.Struct!(u8, u16): DescribePair,
    }
}

check_components! {
    App {
        ShapeDescriberComponent: [
            Struct! { x: f64, y: f64 },
            Struct!(u8, u16),
        ],
    }
}

#[test]
fn a_shape_keys_an_open_dispatch_entry() {
    assert_eq!(
        CanDescribeShape::<Struct! { x: f64, y: f64 }>::describe_shape(&App),
        "point"
    );
    assert_eq!(
        CanDescribeShape::<Struct!(u8, u16)>::describe_shape(&App),
        "pair"
    );
}
