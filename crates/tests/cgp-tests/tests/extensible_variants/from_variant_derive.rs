//! `#[derive(FromVariant)]` on its own: the variant-construction slice, and the
//! simplest derive in the extensible-data family.
//!
//! The snapshot shows the whole of what it emits — one `FromVariant` impl per
//! variant and nothing more. There is no companion type and no presence
//! tracking, because construction needs neither: each impl just wraps a payload
//! in its variant, keyed by the variant name's `Symbol!`.
//!
//! What that buys is a constructor a generic caller can select by tag. The test
//! below stays generic over the tag, so one function can build either variant,
//! which a `Shape::Circle(..)` call site cannot.
//!
//! See cgp-knowledge-base/cgp/reference/derives/derive_from_variant.md and
//! cgp-knowledge-base/cgp/reference/traits/from_variant.md.

use core::marker::PhantomData;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_from_variant;

#[derive(Debug, Eq, PartialEq)]
pub struct Circle {
    pub radius: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

snapshot_derive_from_variant! {
    #[derive(FromVariant)]
    #[derive(Debug, Eq, PartialEq)]
    pub enum Shape {
        Circle(Circle),
        Rectangle(Rectangle),
    }

    expand_shape(output) {
        insta::assert_snapshot!(output, @"
        impl FromVariant<
            Symbol<
                6,
                Chars<'C', Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>>,
            >,
        > for Shape {
            type Value = Circle;
            fn from_variant(
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'C',
                            Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>,
                        >,
                    >,
                >,
                value: Self::Value,
            ) -> Self {
                Self::Circle(value)
            }
        }
        impl FromVariant<
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
        > for Shape {
            type Value = Rectangle;
            fn from_variant(
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
                value: Self::Value,
            ) -> Self {
                Self::Rectangle(value)
            }
        }
        ")
    }
}

/// Build a `Shape` without naming which variant — the caller picks it with a tag.
fn wrap<Tag>(tag: PhantomData<Tag>, value: <Shape as FromVariant<Tag>>::Value) -> Shape
where
    Shape: FromVariant<Tag>,
{
    Shape::from_variant(tag, value)
}

#[test]
fn test_construct_each_variant_by_tag() {
    assert_eq!(
        wrap(PhantomData::<Symbol!("Circle")>, Circle { radius: 2 }),
        Shape::Circle(Circle { radius: 2 })
    );

    assert_eq!(
        wrap(
            PhantomData::<Symbol!("Rectangle")>,
            Rectangle {
                width: 3,
                height: 4,
            }
        ),
        Shape::Rectangle(Rectangle {
            width: 3,
            height: 4,
        })
    );
}
