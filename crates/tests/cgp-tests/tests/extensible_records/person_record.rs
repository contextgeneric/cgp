//! `#[derive(CgpData)]` with multi-character field names, and building a larger
//! record from a smaller one and from a handler pipeline.
//!
//! The snapshot pins how a field name longer than a few characters
//! (`first_name`, `last_name`) expands into its `Symbol<N, Chars<...>>` spine —
//! the leading `N` is the length. The runtime tests then show the extensible
//! builder absorbing a `Person` into an `Employee` builder (via `build_from`),
//! filling the remaining field either explicitly (`build_field`) or from another
//! record, and driving the same build through a `BuildWithHandlers` pipeline.
//!
//! `Person` owns the snapshot here; `Employee`/`EmployeeId` are plain `CgpData`
//! fixtures whose expansion is already pinned by `record_derive`.
//!
//! See docs/reference/derives/derive_cgp_data.md and
//! docs/reference/traits/has_builder.md.

use core::marker::PhantomData;

use cgp::core::field::impls::CanBuildFrom;
use cgp::extra::dispatch::{BuildAndMerge, BuildAndSetField, BuildWithHandlers};
use cgp::extra::handler::Computer;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_cgp_data;

snapshot_derive_cgp_data! {
    #[derive(CgpData)]
    pub struct Person {
        pub first_name: String,
        pub last_name: String,
    }

    expand_person(output) {
        insta::assert_snapshot!(output, @"
        impl HasField<
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
        > for Person {
            type Value = String;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<
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
        impl HasFieldMut<
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
        > for Person {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<
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
            ) -> &mut Self::Value {
                &mut self.first_name
            }
        }
        impl HasField<
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
        > for Person {
            type Value = String;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<
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
        impl HasFieldMut<
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
        > for Person {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<
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
            ) -> &mut Self::Value {
                &mut self.last_name
            }
        }
        impl HasFields for Person {
            type Fields = Cons<
                Field<
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
                    String,
                >,
                Cons<
                    Field<
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
                        String,
                    >,
                    Nil,
                >,
            >;
        }
        impl HasFieldsRef for Person {
            type FieldsRef<'__a> = Cons<
                Field<
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
                    &'__a String,
                >,
                Cons<
                    Field<
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
                        &'__a String,
                    >,
                    Nil,
                >,
            >
            where
                Self: '__a;
        }
        impl FromFields for Person {
            fn from_fields(Cons(first_name, Cons(last_name, Nil)): Self::Fields) -> Self {
                Self {
                    first_name: first_name.value,
                    last_name: last_name.value,
                }
            }
        }
        impl ToFields for Person {
            fn to_fields(self) -> Self::Fields {
                Cons(self.first_name.into(), Cons(self.last_name.into(), Nil))
            }
        }
        impl ToFieldsRef for Person {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                Cons((&self.first_name).into(), Cons((&self.last_name).into(), Nil))
            }
        }
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

#[derive(CgpData)]
pub struct Employee {
    pub employee_id: u64,
    pub first_name: String,
    pub last_name: String,
}

#[derive(CgpData)]
pub struct EmployeeId {
    pub employee_id: u64,
}

#[cgp_producer]
pub fn build_person() -> Person {
    Person {
        first_name: "John".to_owned(),
        last_name: "Smith".to_owned(),
    }
}

#[cgp_producer]
pub fn build_employee_id() -> u64 {
    1
}

#[test]
fn test_person() {
    let person = Person {
        first_name: "John".to_owned(),
        last_name: "Smith".to_owned(),
    };

    let _employee: Employee = Employee::builder() // PartialEmployee<IsNothing, IsNothing, IsNothing>
        .build_from(person) // PartialEmployee<IsNothing, IsPresent, IsPresent>
        .build_field(PhantomData::<Symbol!("employee_id")>, 1) // PartialEmployee<IsPresent, IsPresent, IsPresent>
        .finalize_build(); // Person
}

#[test]
fn test_person2() {
    let person = Person {
        first_name: "John".to_owned(),
        last_name: "Smith".to_owned(),
    };

    let employee_id = EmployeeId { employee_id: 1 };

    let _employee = Employee::builder() // PartialEmployee<IsNothing, IsNothing, IsNothing>
        .build_from(person) // PartialEmployee<IsNothing, IsPresent, IsPresent>
        .build_from(employee_id) // PartialEmployee<IsPresent, IsPresent, IsPresent>
        .finalize_build(); // Person
}

#[test]
fn test_build_with_handler() {
    let _employee = BuildWithHandlers::<
        Employee,
        Product![BuildAndMerge<BuildPerson>, BuildAndSetField<Symbol!("employee_id"), BuildEmployeeId>],
    >::compute(&(), PhantomData::<()>, ());
}
