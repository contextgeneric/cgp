//! `#[derive(HasFields)]` on a generic enum with a lifetime and a type
//! parameter: the derive lifts the enum's generics onto every generated impl,
//! and a variant payload that is itself a reference (`&'a Name`) appears
//! verbatim in the field list (and gains a second borrow in `FieldsRef`).
//!
//! This concept owns the enum expansion of the `HasFields` derive; this file
//! pins the generic-enum variant of that expansion.
//!
//! See docs/reference/derives/derive_has_fields.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_fields;

snapshot_derive_has_fields! {
    #[derive(HasFields)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Person<'a, Name> {
        Anonymous(u32),
        Named(&'a Name),
    }

    expand_person(output) {
        insta::assert_snapshot!(output, @"
        impl<'a, Name> HasFields for Person<'a, Name> {
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
                        &'a Name,
                    >,
                    Void,
                >,
            >;
        }
        impl<'a, Name> HasFieldsRef for Person<'a, Name> {
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
                        &'__a &'a Name,
                    >,
                    Void,
                >,
            >
            where
                Self: '__a;
        }
        impl<'a, Name> FromFields for Person<'a, Name> {
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
        impl<'a, Name> ToFields for Person<'a, Name> {
            fn to_fields(self) -> Self::Fields {
                match self {
                    Self::Anonymous(field) => Either::Left(field.into()),
                    Self::Named(field) => Either::Right(Either::Left(field.into())),
                }
            }
        }
        impl<'a, Name> ToFieldsRef for Person<'a, Name> {
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
fn test_generic_enum() {
    {
        let person_a1: Person<String> = Person::Anonymous(42);

        let person_a2 = person_a1.clone().to_fields();
        assert_eq!(person_a2, Either::Left(42.into()));

        let person_a3 = Person::from_fields(person_a2);
        assert_eq!(person_a3, person_a1);

        let person_a4 = person_a1.to_fields_ref();
        assert_eq!(person_a4, Either::Left((&42).into()));
    }

    {
        let name = "Alice".to_owned();

        let person_b1 = Person::Named(&name);

        let person_b2 = person_b1.clone().to_fields();
        assert_eq!(person_b2, Either::Right(Either::Left((&name).into())));

        let person_b3 = Person::from_fields(person_b2);
        assert_eq!(person_b3, person_b1);

        let person_b4 = person_b1.to_fields_ref();
        assert_eq!(person_b4, Either::Right(Either::Left((&&name).into())));
    }
}
