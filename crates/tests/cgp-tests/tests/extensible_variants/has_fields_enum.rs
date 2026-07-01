//! `#[derive(HasFields)]` on a plain enum: the derive exposes the variants as a
//! `Sum!` of `Field<Symbol!("Variant"), Payload>` entries and generates the
//! `HasFields`/`HasFieldsRef`/`FromFields`/`ToFields`/`ToFieldsRef` impls that
//! convert an enum value to and from that structural sum. Unlike
//! `#[derive(CgpData)]`, `HasFields` stops at the field list and does not derive
//! the extractor/`FromVariant` machinery.
//!
//! This concept owns the enum expansion of the `HasFields` derive.
//!
//! See docs/reference/derives/derive_has_fields.md and docs/reference/macros/sum.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_fields;

snapshot_derive_has_fields! {
    #[derive(HasFields)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Person {
        Anonymous(u32),
        Named(String),
    }

    expand_person(output) {
        insta::assert_snapshot!(output, @"
        impl HasFields for Person {
            type Fields = Either<
                Field<
                    Symbol<
                        9,
                        Chars<
                            'A',
                            Chars<
                                'n',
                                Chars<
                                    'o',
                                    Chars<
                                        'n',
                                        Chars<
                                            'y',
                                            Chars<'m', Chars<'o', Chars<'u', Chars<'s', Nil>>>>,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                    u32,
                >,
                Either<
                    Field<
                        Symbol<
                            5,
                            Chars<'N', Chars<'a', Chars<'m', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                        String,
                    >,
                    Void,
                >,
            >;
        }
        impl HasFieldsRef for Person {
            type FieldsRef<'__a> = Either<
                Field<
                    Symbol<
                        9,
                        Chars<
                            'A',
                            Chars<
                                'n',
                                Chars<
                                    'o',
                                    Chars<
                                        'n',
                                        Chars<
                                            'y',
                                            Chars<'m', Chars<'o', Chars<'u', Chars<'s', Nil>>>>,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                    &'__a u32,
                >,
                Either<
                    Field<
                        Symbol<
                            5,
                            Chars<'N', Chars<'a', Chars<'m', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                        &'__a String,
                    >,
                    Void,
                >,
            >
            where
                Self: '__a;
        }
        impl FromFields for Person {
            fn from_fields(rest: Self::Fields) -> Self {
                match rest {
                    Either::Left(field) => {
                        let field = field.value;
                        Self::Anonymous(field)
                    }
                    Either::Right(rest) => {
                        match rest {
                            Either::Left(field) => {
                                let field = field.value;
                                Self::Named(field)
                            }
                            Either::Right(rest) => match rest {}
                        }
                    }
                }
            }
        }
        impl ToFields for Person {
            fn to_fields(self) -> Self::Fields {
                match self {
                    Self::Anonymous(field) => Either::Left(field.into()),
                    Self::Named(field) => Either::Right(Either::Left(field.into())),
                }
            }
        }
        impl ToFieldsRef for Person {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                match self {
                    Self::Anonymous(field) => Either::Left(field.into()),
                    Self::Named(field) => Either::Right(Either::Left(field.into())),
                }
            }
        }
        ")
    }
}

#[test]
fn test_simple_enum() {
    {
        let person_a1 = Person::Anonymous(42);

        let person_a2 = person_a1.clone().to_fields();
        assert_eq!(person_a2, Either::Left(42.into()));

        let person_a3 = Person::from_fields(person_a2);
        assert_eq!(person_a3, person_a1);

        let person_a4 = person_a1.to_fields_ref();
        assert_eq!(person_a4, Either::Left((&42).into()));
    }

    {
        let name = "Alice".to_owned();

        let person_b1 = Person::Named(name.clone());

        let person_b2 = person_b1.clone().to_fields();
        assert_eq!(person_b2, Either::Right(Either::Left(name.clone().into())));

        let person_b3 = Person::from_fields(person_b2);
        assert_eq!(person_b3, person_b1);

        let person_b4 = person_b1.to_fields_ref();
        assert_eq!(person_b4, Either::Right(Either::Left((&name).into())));
    }
}
