//! A `Struct!` is an ordinary type, so it can stand anywhere a type can: an
//! associated-type binding, a `where` clause, an argument or return type, the
//! self type of an impl, an associated type, and a wiring entry. The brace form
//! is the case worth pinning in an impl header, where its body sits directly
//! before the impl's own braces.
//!
//! Inside `#[cgp_component]` and `#[cgp_impl]`, `Self` written in a shape's body
//! is rewritten to the context like any other `Self`, because that rewrite
//! reaches into macro tokens.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/struct.md.

use cgp::prelude::*;

use crate::shape_macros::assertions::assert_same_type;

#[derive(Debug, PartialEq, HasFields)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

pub trait Describe {
    fn describe() -> &'static str;
}

impl Describe for Struct! { name: String, age: u8 } {
    fn describe() -> &'static str {
        "a person's shape"
    }
}

impl Describe for Struct!(u8, u16) {
    fn describe() -> &'static str {
        "a pair's shape"
    }
}

pub fn describe_shape_of<T>() -> &'static str
where
    T: HasFields<Fields = Struct! { name: String, age: u8 }>,
{
    <T::Fields as Describe>::describe()
}

pub fn rebuild<T>(fields: Struct! { name: String, age: u8 }) -> T
where
    T: FromFields<Fields = Struct! { name: String, age: u8 }>,
{
    T::from_fields(fields)
}

pub fn empty_shape() -> Struct! {} {
    Nil
}

pub trait HasShape {
    type Shape;
}

impl HasShape for Person {
    type Shape = Struct! { name: String, age: u8 };
}

#[test]
fn a_shape_is_usable_in_bounds_and_signatures() {
    assert_eq!(describe_shape_of::<Person>(), "a person's shape");
    assert_eq!(<Struct!(u8, u16) as Describe>::describe(), "a pair's shape");

    let person: Person = rebuild(product!["Carol".to_owned().into(), 25u8.into()]);
    assert_eq!(person.age, 25);

    assert_eq!(empty_shape(), Nil);

    assert_same_type::<<Person as HasShape>::Shape, <Person as HasFields>::Fields>();
}

#[cgp_type]
pub trait HasPointType {
    type Point;
}

#[cgp_component(ShapeMarker)]
pub trait CanMarkShape {
    fn shape_marker(&self) -> PhantomData<Struct! { context: PhantomData<Self>, size: usize }>;
}

#[cgp_impl(new MarkShape)]
impl ShapeMarker {
    fn shape_marker(&self) -> PhantomData<Struct! { context: PhantomData<Self>, size: usize }> {
        PhantomData
    }
}

pub struct App;

delegate_components! {
    App {
        PointTypeProviderComponent: UseType<Struct! { x: f64, y: f64 }>,
        ShapeMarkerComponent: MarkShape,
    }
}

check_components! {
    App {
        PointTypeProviderComponent,
        ShapeMarkerComponent,
    }
}

#[test]
fn a_shape_is_usable_in_wiring_and_providers() {
    assert_same_type::<<App as HasPointType>::Point, Struct! { x: f64, y: f64 }>();

    let _marker: PhantomData<Struct! { context: PhantomData<App>, size: usize }> =
        App.shape_marker();
}
