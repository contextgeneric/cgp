//! `#[derive(HasFields)]` on an enum whose variants use **every variant shape**.
//!
//! The variant derives that deconstruct an enum — `#[derive(ExtractField)]` and
//! `#[derive(FromVariant)]`, and therefore `#[derive(CgpVariant)]` and
//! `#[derive(CgpData)]` — require every variant to be a single-unnamed-field
//! tuple variant, because each has to name one payload type. `HasFields` has no
//! such requirement: it only *describes* each variant, so it accepts all four
//! shapes and nests each variant's own fields as a product inside that
//! variant's `Field` entry.
//!
//! The four shapes map as follows, and this file pins each:
//!
//! - a unit variant becomes the empty product `Nil`;
//! - a single-unnamed-field variant becomes the payload type directly, the same
//!   newtype special case a one-field tuple struct gets;
//! - a multi-field tuple variant becomes a product keyed by `Index<N>`;
//! - a named-field (struct-style) variant becomes a product keyed by `Symbol!`.
//!
//! See cgp-knowledge-base/cgp/reference/derives/derive_has_fields.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_fields;

snapshot_derive_has_fields! {
    #[derive(HasFields)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Shape {
        Empty,
        Circle(u32),
        Rectangle(u32, u32),
        Triangle { base: u32, height: u32 },
    }

    expand_shape(output) {
        insta::assert_snapshot!(output, @"
        impl HasFields for Shape {
            type Fields = Either<
                Field<
                    Symbol<5, Chars<'E', Chars<'m', Chars<'p', Chars<'t', Chars<'y', Nil>>>>>>,
                    Nil,
                >,
                Either<
                    Field<
                        Symbol<
                            6,
                            Chars<
                                'C',
                                Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>,
                            >,
                        >,
                        u32,
                    >,
                    Either<
                        Field<
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
                            Cons<Field<Index<0>, u32>, Cons<Field<Index<1>, u32>, Nil>>,
                        >,
                        Either<
                            Field<
                                Symbol<
                                    8,
                                    Chars<
                                        'T',
                                        Chars<
                                            'r',
                                            Chars<
                                                'i',
                                                Chars<
                                                    'a',
                                                    Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                                Cons<
                                    Field<
                                        Symbol<
                                            4,
                                            Chars<'b', Chars<'a', Chars<'s', Chars<'e', Nil>>>>,
                                        >,
                                        u32,
                                    >,
                                    Cons<
                                        Field<
                                            Symbol<
                                                6,
                                                Chars<
                                                    'h',
                                                    Chars<
                                                        'e',
                                                        Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>,
                                                    >,
                                                >,
                                            >,
                                            u32,
                                        >,
                                        Nil,
                                    >,
                                >,
                            >,
                            Void,
                        >,
                    >,
                >,
            >;
        }
        impl HasFieldsRef for Shape {
            type FieldsRef<'__a> = Either<
                Field<
                    Symbol<5, Chars<'E', Chars<'m', Chars<'p', Chars<'t', Chars<'y', Nil>>>>>>,
                    Nil,
                >,
                Either<
                    Field<
                        Symbol<
                            6,
                            Chars<
                                'C',
                                Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>,
                            >,
                        >,
                        &'__a u32,
                    >,
                    Either<
                        Field<
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
                            Cons<
                                Field<Index<0>, &'__a u32>,
                                Cons<Field<Index<1>, &'__a u32>, Nil>,
                            >,
                        >,
                        Either<
                            Field<
                                Symbol<
                                    8,
                                    Chars<
                                        'T',
                                        Chars<
                                            'r',
                                            Chars<
                                                'i',
                                                Chars<
                                                    'a',
                                                    Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                                Cons<
                                    Field<
                                        Symbol<
                                            4,
                                            Chars<'b', Chars<'a', Chars<'s', Chars<'e', Nil>>>>,
                                        >,
                                        &'__a u32,
                                    >,
                                    Cons<
                                        Field<
                                            Symbol<
                                                6,
                                                Chars<
                                                    'h',
                                                    Chars<
                                                        'e',
                                                        Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>,
                                                    >,
                                                >,
                                            >,
                                            &'__a u32,
                                        >,
                                        Nil,
                                    >,
                                >,
                            >,
                            Void,
                        >,
                    >,
                >,
            >
            where
                Self: '__a;
        }
        impl FromFields for Shape {
            fn from_fields(rest: Self::Fields) -> Self {
                match rest {
                    Either::Left(field) => {
                        let Nil = field.value;
                        Self::Empty
                    }
                    Either::Right(rest) => {
                        match rest {
                            Either::Left(field) => {
                                let field = field.value;
                                Self::Circle(field)
                            }
                            Either::Right(rest) => {
                                match rest {
                                    Either::Left(field) => {
                                        let Cons(field_1, Cons(field_0, Nil)) = field.value;
                                        Self::Rectangle(field_1.value, field_0.value)
                                    }
                                    Either::Right(rest) => {
                                        match rest {
                                            Either::Left(field) => {
                                                let Cons(base, Cons(height, Nil)) = field.value;
                                                Self::Triangle {
                                                    base: base.value,
                                                    height: height.value,
                                                }
                                            }
                                            Either::Right(rest) => match rest {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        impl ToFields for Shape {
            fn to_fields(self) -> Self::Fields {
                match self {
                    Self::Empty => Either::Left(Nil.into()),
                    Self::Circle(field) => Either::Right(Either::Left(field.into())),
                    Self::Rectangle(field_0, field_1) => {
                        Either::Right(
                            Either::Right(
                                Either::Left(
                                    Cons(field_0.into(), Cons(field_1.into(), Nil)).into(),
                                ),
                            ),
                        )
                    }
                    Self::Triangle { base, height } => {
                        Either::Right(
                            Either::Right(
                                Either::Right(
                                    Either::Left(
                                        Cons(base.into(), Cons(height.into(), Nil)).into(),
                                    ),
                                ),
                            ),
                        )
                    }
                }
            }
        }
        impl ToFieldsRef for Shape {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                match self {
                    Self::Empty => Either::Left(Nil.into()),
                    Self::Circle(field) => Either::Right(Either::Left(field.into())),
                    Self::Rectangle(field_0, field_1) => {
                        Either::Right(
                            Either::Right(
                                Either::Left(
                                    Cons(field_0.into(), Cons(field_1.into(), Nil)).into(),
                                ),
                            ),
                        )
                    }
                    Self::Triangle { base, height } => {
                        Either::Right(
                            Either::Right(
                                Either::Right(
                                    Either::Left(
                                        Cons(base.into(), Cons(height.into(), Nil)).into(),
                                    ),
                                ),
                            ),
                        )
                    }
                }
            }
        }
        ")
    }
}

#[test]
fn test_unit_variant() {
    let shape1 = Shape::Empty;

    let fields = shape1.clone().to_fields();
    assert_eq!(fields, Either::Left(Nil.into()));

    let shape2 = Shape::from_fields(fields);
    assert_eq!(shape1, shape2);
}

#[test]
fn test_newtype_variant() {
    let shape1 = Shape::Circle(2);

    // The payload is the inner type directly, not a one-element product.
    let fields = shape1.clone().to_fields();
    assert_eq!(fields, Either::Right(Either::Left(2.into())));

    let shape2 = Shape::from_fields(fields);
    assert_eq!(shape1, shape2);
}

#[test]
fn test_multi_field_tuple_variant() {
    let shape1 = Shape::Rectangle(3, 4);

    // Positional fields are keyed by `Index<N>`, in declaration order.
    let fields = shape1.clone().to_fields();
    assert_eq!(
        fields,
        Either::Right(Either::Right(Either::Left(
            Cons(3.into(), Cons(4.into(), Nil)).into()
        )))
    );

    let shape2 = Shape::from_fields(fields);
    assert_eq!(shape1, shape2);
}

#[test]
fn test_named_field_variant() {
    let shape1 = Shape::Triangle { base: 6, height: 5 };

    // Named fields are keyed by `Symbol!`, in declaration order.
    let fields = shape1.clone().to_fields();
    assert_eq!(
        fields,
        Either::Right(Either::Right(Either::Right(Either::Left(
            Cons(6.into(), Cons(5.into(), Nil)).into()
        ))))
    );

    let shape2 = Shape::from_fields(fields);
    assert_eq!(shape1, shape2);
}

#[test]
fn test_borrowed_fields_across_every_shape() {
    // `to_fields_ref` borrows each payload in place, whatever the variant shape.
    assert_eq!(Shape::Empty.to_fields_ref(), Either::Left(Nil.into()));

    assert_eq!(
        Shape::Circle(2).to_fields_ref(),
        Either::Right(Either::Left((&2).into()))
    );

    assert_eq!(
        Shape::Rectangle(3, 4).to_fields_ref(),
        Either::Right(Either::Right(Either::Left(
            Cons((&3).into(), Cons((&4).into(), Nil)).into()
        )))
    );

    assert_eq!(
        Shape::Triangle { base: 6, height: 5 }.to_fields_ref(),
        Either::Right(Either::Right(Either::Right(Either::Left(
            Cons((&6).into(), Cons((&5).into(), Nil)).into()
        ))))
    );
}
