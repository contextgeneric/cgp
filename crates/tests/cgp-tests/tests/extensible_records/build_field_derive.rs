//! `#[derive(BuildField)]` on its own: the builder slice of the record
//! machinery, and nothing else.
//!
//! The other record derives are supersets of this one, so it is easy to miss
//! what it emits by itself. The snapshot is the answer, and its shape is the
//! point: a `__Partial…` companion struct, the `HasBuilder`/`IntoBuilder` entry
//! points, `PartialData`, the all-present `FinalizeBuild`, a per-field
//! `UpdateField`, and a per-field `HasField` on the *partial* type — with no
//! `HasField` getters on the original struct and no `HasFields` representation
//! impls, which `#[derive(HasField)]` and `#[derive(HasFields)]` supply.
//!
//! Two capabilities come from field-crate blanket impls over the generated
//! `UpdateField`, rather than from the derive, and this file exercises both in
//! opposite directions: `BuildField` sets an absent field (`IsNothing` to
//! `IsPresent`) and `TakeField` removes a present one (`IsPresent` back to
//! `IsNothing`). Because presence lives in the partial type's parameters, a
//! premature `finalize_build` is a compile error rather than a runtime failure.
//!
//! See cgp-knowledge-base/cgp/reference/derives/derive_build_field.md and
//! cgp-knowledge-base/cgp/reference/traits/has_builder.md.

use core::marker::PhantomData;

use cgp::core::field::traits::TakeField;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_build_field;

snapshot_derive_build_field! {
    #[derive(BuildField)]
    #[derive(Debug, Eq, PartialEq)]
    pub struct Person {
        pub first_name: String,
        pub last_name: String,
    }

    expand_person(output) {
        insta::assert_snapshot!(output, @"
        pub struct __PartialPerson<__F0__: MapType, __F1__: MapType> {
            pub first_name: <__F0__ as MapType>::Map<String>,
            pub last_name: <__F1__ as MapType>::Map<String>,
        }
        impl HasBuilder for Person {
            type Builder = __PartialPerson<IsNothing, IsNothing>;
            fn builder() -> Self::Builder {
                __PartialPerson {
                    first_name: (),
                    last_name: (),
                }
            }
        }
        impl IntoBuilder for Person {
            type Builder = __PartialPerson<IsPresent, IsPresent>;
            fn into_builder(self) -> Self::Builder {
                __PartialPerson {
                    first_name: self.first_name,
                    last_name: self.last_name,
                }
            }
        }
        impl<__F0__: MapType, __F1__: MapType> PartialData for __PartialPerson<__F0__, __F1__> {
            type Target = Person;
        }
        impl FinalizeBuild for __PartialPerson<IsPresent, IsPresent> {
            fn finalize_build(self) -> Self::Target {
                Person {
                    first_name: self.first_name,
                    last_name: self.last_name,
                }
            }
        }
        impl<
            __M1__: MapType,
            __M2__: MapType,
            __F1__: MapType,
        > UpdateField<
            Symbol<
                10,
                Chars<
                    'f',
                    Chars<
                        'i',
                        Chars<
                            'r',
                            Chars<
                                's',
                                Chars<
                                    't',
                                    Chars<
                                        '_',
                                        Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
            __M2__,
        > for __PartialPerson<__M1__, __F1__> {
            type Value = String;
            type Mapper = __M1__;
            type Output = __PartialPerson<__M2__, __F1__>;
            fn update_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        10,
                        Chars<
                            'f',
                            Chars<
                                'i',
                                Chars<
                                    'r',
                                    Chars<
                                        's',
                                        Chars<
                                            't',
                                            Chars<
                                                '_',
                                                Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
                value: __M2__::Map<Self::Value>,
            ) -> (__M1__::Map<Self::Value>, Self::Output) {
                (
                    self.first_name,
                    __PartialPerson {
                        first_name: value,
                        last_name: self.last_name,
                    },
                )
            }
        }
        impl<
            __F0__: MapType,
            __M1__: MapType,
            __M2__: MapType,
        > UpdateField<
            Symbol<
                9,
                Chars<
                    'l',
                    Chars<
                        'a',
                        Chars<
                            's',
                            Chars<
                                't',
                                Chars<'_', Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                            >,
                        >,
                    >,
                >,
            >,
            __M2__,
        > for __PartialPerson<__F0__, __M1__> {
            type Value = String;
            type Mapper = __M1__;
            type Output = __PartialPerson<__F0__, __M2__>;
            fn update_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        9,
                        Chars<
                            'l',
                            Chars<
                                'a',
                                Chars<
                                    's',
                                    Chars<
                                        't',
                                        Chars<
                                            '_',
                                            Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
                value: __M2__::Map<Self::Value>,
            ) -> (__M1__::Map<Self::Value>, Self::Output) {
                (
                    self.last_name,
                    __PartialPerson {
                        first_name: self.first_name,
                        last_name: value,
                    },
                )
            }
        }
        impl<
            __F1__: MapType,
        > HasField<
            Symbol<
                10,
                Chars<
                    'f',
                    Chars<
                        'i',
                        Chars<
                            'r',
                            Chars<
                                's',
                                Chars<
                                    't',
                                    Chars<
                                        '_',
                                        Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        > for __PartialPerson<IsPresent, __F1__> {
            type Value = String;
            fn get_field(
                &self,
                tag: ::core::marker::PhantomData<
                    Symbol<
                        10,
                        Chars<
                            'f',
                            Chars<
                                'i',
                                Chars<
                                    'r',
                                    Chars<
                                        's',
                                        Chars<
                                            't',
                                            Chars<
                                                '_',
                                                Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            ) -> &Self::Value {
                &self.first_name
            }
        }
        impl<
            __F0__: MapType,
        > HasField<
            Symbol<
                9,
                Chars<
                    'l',
                    Chars<
                        'a',
                        Chars<
                            's',
                            Chars<
                                't',
                                Chars<'_', Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                            >,
                        >,
                    >,
                >,
            >,
        > for __PartialPerson<__F0__, IsPresent> {
            type Value = String;
            fn get_field(
                &self,
                tag: ::core::marker::PhantomData<
                    Symbol<
                        9,
                        Chars<
                            'l',
                            Chars<
                                'a',
                                Chars<
                                    's',
                                    Chars<
                                        't',
                                        Chars<
                                            '_',
                                            Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            ) -> &Self::Value {
                &self.last_name
            }
        }
        ")
    }
}

#[test]
fn test_build_field_by_field() {
    let person: Person = Person::builder()
        .build_field(PhantomData::<Symbol!("first_name")>, "Alice".to_owned())
        .build_field(PhantomData::<Symbol!("last_name")>, "Anderson".to_owned())
        .finalize_build();

    assert_eq!(
        person,
        Person {
            first_name: "Alice".to_owned(),
            last_name: "Anderson".to_owned(),
        }
    );
}

#[test]
fn test_read_a_field_back_out_of_a_partial_value() {
    // The per-field `HasField` impl on the partial type is in scope only once
    // that field is present, so a field can be read back mid-build.
    let partial =
        Person::builder().build_field(PhantomData::<Symbol!("first_name")>, "Alice".to_owned());

    assert_eq!(
        partial.get_field(PhantomData::<Symbol!("first_name")>),
        "Alice"
    );
}

#[test]
fn test_round_trip_through_into_builder_and_take_field() {
    let person1 = Person {
        first_name: "Alice".to_owned(),
        last_name: "Anderson".to_owned(),
    };

    // `IntoBuilder` goes the other way from `HasBuilder`: an existing value
    // becomes an all-present partial value.
    let builder = person1.into_builder();

    // `TakeField` is `BuildField` reversed — it pulls a present field out and
    // hands back the value alongside a partial value with that field absent.
    let (first_name, remainder) = builder.take_field(PhantomData::<Symbol!("first_name")>);
    assert_eq!(first_name, "Alice");

    // Putting it back restores the all-present configuration, which is the only
    // one `finalize_build` is implemented for.
    let person2 = remainder
        .build_field(PhantomData::<Symbol!("first_name")>, first_name)
        .finalize_build();

    assert_eq!(
        person2,
        Person {
            first_name: "Alice".to_owned(),
            last_name: "Anderson".to_owned(),
        }
    );
}
