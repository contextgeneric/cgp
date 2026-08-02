//! `#[derive(ExtractField)]` on its own: the extractor slice of the variant
//! machinery, and nothing else.
//!
//! The snapshot is the point, and what it does *not* contain matters as much as
//! what it does: two `__Partial…` companion enums (one owned, one borrowed),
//! `PartialData` for each, the three `HasExtractor` accessors, an all-`IsVoid`
//! `FinalizeExtract` for each, and a per-variant `ExtractField` — but no
//! `HasFields` representation impls and no `FromVariant` constructors, which
//! `#[derive(HasFields)]` and `#[derive(FromVariant)]` supply. So a value is
//! built here with the enum's ordinary constructor, not generically.
//!
//! The runtime tests walk the chain in both outcomes. Each failed
//! `extract_field` hands back a *remainder* whose type has one more variant
//! marked `IsVoid`, and once every variant has been tried the remainder is
//! uninhabited — which is what lets `finalize_extract_result` close the chain
//! with no wildcard arm.
//!
//! See cgp-knowledge-base/cgp/reference/derives/derive_extract_field.md and
//! cgp-knowledge-base/cgp/reference/traits/extract_field.md.

use core::marker::PhantomData;

use cgp::core::field::traits::FinalizeExtractResult;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_extract_field;

#[derive(Debug, Eq, PartialEq)]
pub struct Circle {
    pub radius: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

snapshot_derive_extract_field! {
    #[derive(ExtractField)]
    #[derive(Debug, Eq, PartialEq)]
    pub enum Shape {
        Circle(Circle),
        Rectangle(Rectangle),
    }

    expand_shape(output) {
        insta::assert_snapshot!(output, @"
        pub enum __PartialShape<__F0__: MapType, __F1__: MapType> {
            Circle(<__F0__ as MapType>::Map<Circle>),
            Rectangle(<__F1__ as MapType>::Map<Rectangle>),
        }
        pub enum __PartialRefShape<'__a__, __R__: MapTypeRef, __F0__: MapType, __F1__: MapType> {
            Circle(<__F0__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, Circle>>),
            Rectangle(<__F1__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, Rectangle>>),
        }
        impl<__F0__: MapType, __F1__: MapType> PartialData for __PartialShape<__F0__, __F1__> {
            type Target = Shape;
        }
        impl<'__a__, __R__: MapTypeRef, __F0__: MapType, __F1__: MapType> PartialData
        for __PartialRefShape<'__a__, __R__, __F0__, __F1__> {
            type Target = Shape;
        }
        impl HasExtractor for Shape {
            type Extractor = __PartialShape<IsPresent, IsPresent>;
            fn to_extractor(self) -> Self::Extractor {
                match self {
                    Self::Circle(value) => __PartialShape::Circle(value),
                    Self::Rectangle(value) => __PartialShape::Rectangle(value),
                }
            }
            fn from_extractor(extractor: Self::Extractor) -> Self {
                match extractor {
                    __PartialShape::Circle(value) => Self::Circle(value),
                    __PartialShape::Rectangle(value) => Self::Rectangle(value),
                }
            }
        }
        impl HasExtractorRef for Shape {
            type ExtractorRef<'__a__> = __PartialRefShape<'__a__, IsRef, IsPresent, IsPresent>
            where
                Self: '__a__;
            fn extractor_ref<'__a__>(&'__a__ self) -> Self::ExtractorRef<'__a__> {
                match self {
                    Self::Circle(value) => __PartialRefShape::Circle(value),
                    Self::Rectangle(value) => __PartialRefShape::Rectangle(value),
                }
            }
        }
        impl HasExtractorMut for Shape {
            type ExtractorMut<'__a__> = __PartialRefShape<'__a__, IsMut, IsPresent, IsPresent>
            where
                Self: '__a__;
            fn extractor_mut<'__a__>(&'__a__ mut self) -> Self::ExtractorMut<'__a__> {
                match self {
                    Self::Circle(value) => __PartialRefShape::Circle(value),
                    Self::Rectangle(value) => __PartialRefShape::Rectangle(value),
                }
            }
        }
        impl FinalizeExtract for __PartialShape<IsVoid, IsVoid> {
            fn finalize_extract<__T__>(self) -> __T__ {
                match self {}
            }
        }
        impl<'__a__, __R__: MapTypeRef> FinalizeExtract
        for __PartialRefShape<'__a__, __R__, IsVoid, IsVoid> {
            fn finalize_extract<__T__>(self) -> __T__ {
                match self {}
            }
        }
        impl<
            __F1__: MapType,
        > ExtractField<
            Symbol<
                6,
                Chars<'C', Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>>,
            >,
        > for __PartialShape<IsPresent, __F1__> {
            type Value = Circle;
            type Remainder = __PartialShape<IsVoid, __F1__>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'C',
                            Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialShape::Circle(value) => Ok(value),
                    __PartialShape::Rectangle(value) => Err(__PartialShape::Rectangle(value)),
                }
            }
        }
        impl<
            __F0__: MapType,
        > ExtractField<
            Symbol<
                9,
                Chars<
                    'R',
                    Chars<
                        'e',
                        Chars<
                            'c',
                            Chars<
                                't',
                                Chars<'a', Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>>,
                            >,
                        >,
                    >,
                >,
            >,
        > for __PartialShape<__F0__, IsPresent> {
            type Value = Rectangle;
            type Remainder = __PartialShape<__F0__, IsVoid>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        9,
                        Chars<
                            'R',
                            Chars<
                                'e',
                                Chars<
                                    'c',
                                    Chars<
                                        't',
                                        Chars<
                                            'a',
                                            Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialShape::Circle(value) => Err(__PartialShape::Circle(value)),
                    __PartialShape::Rectangle(value) => Ok(value),
                }
            }
        }
        impl<
            '__a__,
            __R__: MapTypeRef,
            __F1__: MapType,
        > ExtractField<
            Symbol<
                6,
                Chars<'C', Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>>,
            >,
        > for __PartialRefShape<'__a__, __R__, IsPresent, __F1__> {
            type Value = <__R__ as MapTypeRef>::Map<'__a__, Circle>;
            type Remainder = __PartialRefShape<'__a__, __R__, IsVoid, __F1__>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'C',
                            Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialRefShape::Circle(value) => Ok(value),
                    __PartialRefShape::Rectangle(value) => {
                        Err(__PartialRefShape::Rectangle(value))
                    }
                }
            }
        }
        impl<
            '__a__,
            __R__: MapTypeRef,
            __F0__: MapType,
        > ExtractField<
            Symbol<
                9,
                Chars<
                    'R',
                    Chars<
                        'e',
                        Chars<
                            'c',
                            Chars<
                                't',
                                Chars<'a', Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>>,
                            >,
                        >,
                    >,
                >,
            >,
        > for __PartialRefShape<'__a__, __R__, __F0__, IsPresent> {
            type Value = <__R__ as MapTypeRef>::Map<'__a__, Rectangle>;
            type Remainder = __PartialRefShape<'__a__, __R__, __F0__, IsVoid>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        9,
                        Chars<
                            'R',
                            Chars<
                                'e',
                                Chars<
                                    'c',
                                    Chars<
                                        't',
                                        Chars<
                                            'a',
                                            Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialRefShape::Circle(value) => Err(__PartialRefShape::Circle(value)),
                    __PartialRefShape::Rectangle(value) => Ok(value),
                }
            }
        }
        ")
    }
}

/// Walk the chain to whichever variant the value holds, proving exhaustiveness
/// without a fallback arm.
fn describe(shape: Shape) -> String {
    match shape
        .to_extractor()
        .extract_field(PhantomData::<Symbol!("Circle")>)
    {
        Ok(circle) => format!("circle of radius {}", circle.radius),
        Err(remainder) => {
            // `remainder` now has `Circle` ruled out. After the next extraction
            // no variant is left, so the result cannot be an `Err`.
            let rectangle = remainder
                .extract_field(PhantomData::<Symbol!("Rectangle")>)
                .finalize_extract_result();

            format!("{}x{} rectangle", rectangle.width, rectangle.height)
        }
    }
}

#[test]
fn test_extract_the_first_variant() {
    assert_eq!(
        describe(Shape::Circle(Circle { radius: 2 })),
        "circle of radius 2"
    );
}

#[test]
fn test_extract_through_a_remainder() {
    assert_eq!(
        describe(Shape::Rectangle(Rectangle {
            width: 3,
            height: 4,
        })),
        "3x4 rectangle"
    );
}

#[test]
fn test_borrowed_and_mutable_extractors() {
    let mut shape = Shape::Circle(Circle { radius: 2 });

    // `HasExtractorRef` borrows each payload in place.
    let radius = shape
        .extractor_ref()
        .extract_field(PhantomData::<Symbol!("Circle")>)
        .map(|circle| circle.radius)
        .ok();

    assert_eq!(radius, Some(2));

    // `HasExtractorMut` lends each payload mutably.
    if let Ok(circle) = shape
        .extractor_mut()
        .extract_field(PhantomData::<Symbol!("Circle")>)
    {
        circle.radius = 5;
    }

    assert_eq!(shape, Shape::Circle(Circle { radius: 5 }));
}

#[test]
fn test_round_trip_through_the_owned_extractor() {
    let shape1 = Shape::Rectangle(Rectangle {
        width: 3,
        height: 4,
    });

    let extractor = Shape::to_extractor(shape1);
    let shape2 = Shape::from_extractor(extractor);

    assert_eq!(
        shape2,
        Shape::Rectangle(Rectangle {
            width: 3,
            height: 4,
        })
    );
}
