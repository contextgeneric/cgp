pub mod single_name_field {
    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_derive_has_fields;

    snapshot_derive_has_fields! {
        #[derive(HasFields)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Person {
            pub name: String,
        }

        expand_person(output) {
            insta::assert_snapshot!(output, @"
            impl HasFields for Person {
                type Fields = Cons<
                    Field<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>, String>,
                    Nil,
                >;
            }
            impl HasFieldsRef for Person {
                type FieldsRef<'__a> = Cons<
                    Field<
                        Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        &'__a String,
                    >,
                    Nil,
                >
                where
                    Self: '__a;
            }
            impl FromFields for Person {
                fn from_fields(Cons(name, Nil): Self::Fields) -> Self {
                    Self { name: name.value }
                }
            }
            impl ToFields for Person {
                fn to_fields(self) -> Self::Fields {
                    Cons(self.name.into(), Nil)
                }
            }
            impl ToFieldsRef for Person {
                fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
                where
                    Self: '__a,
                {
                    Cons((&self.name).into(), Nil)
                }
            }
            ")
        }
    }

    #[test]
    fn test_single_named_field() {
        let name = "Alice".to_owned();

        let person1 = Person { name: name.clone() };

        let product = person1.clone().to_fields();
        assert_eq!(product, Cons(name.clone().into(), Nil));

        let product_ref = person1.to_fields_ref();
        assert_eq!(product_ref, Cons((&name).into(), Nil));

        let person2 = Person::from_fields(product);

        assert_eq!(person1, person2);
    }
}

pub mod two_named_field {
    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_derive_has_fields;

    snapshot_derive_has_fields! {
        #[derive(HasFields)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Person {
            pub name: String,
            pub age: u8,
        }

        expand_person(output) {
            insta::assert_snapshot!(output, @"
            impl HasFields for Person {
                type Fields = Cons<
                    Field<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>, String>,
                    Cons<Field<Symbol<3, Chars<'a', Chars<'g', Chars<'e', Nil>>>>, u8>, Nil>,
                >;
            }
            impl HasFieldsRef for Person {
                type FieldsRef<'__a> = Cons<
                    Field<
                        Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        &'__a String,
                    >,
                    Cons<Field<Symbol<3, Chars<'a', Chars<'g', Chars<'e', Nil>>>>, &'__a u8>, Nil>,
                >
                where
                    Self: '__a;
            }
            impl FromFields for Person {
                fn from_fields(Cons(name, Cons(age, Nil)): Self::Fields) -> Self {
                    Self {
                        name: name.value,
                        age: age.value,
                    }
                }
            }
            impl ToFields for Person {
                fn to_fields(self) -> Self::Fields {
                    Cons(self.name.into(), Cons(self.age.into(), Nil))
                }
            }
            impl ToFieldsRef for Person {
                fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
                where
                    Self: '__a,
                {
                    Cons((&self.name).into(), Cons((&self.age).into(), Nil))
                }
            }
            ")
        }
    }

    #[test]
    fn test_two_named_field() {
        let name = "Alice".to_owned();

        let person1 = Person {
            name: name.clone(),
            age: 32,
        };

        let product = person1.clone().to_fields();
        assert_eq!(product, Cons(name.clone().into(), Cons(32.into(), Nil)));

        let product_ref = person1.to_fields_ref();
        assert_eq!(product_ref, Cons((&name).into(), Cons((&32).into(), Nil)));

        let person2 = Person::from_fields(product);

        assert_eq!(person1, person2);
    }
}

pub mod generic_struct {
    use core::fmt::Display;

    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_derive_has_fields;

    snapshot_derive_has_fields! {
        #[derive(HasFields)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Person<Name>
        where
            Name: Display,
        {
            pub name: Name,
            pub age: u8,
        }

        expand_person(output) {
            insta::assert_snapshot!(output, @"
            impl<Name> HasFields for Person<Name>
            where
                Name: Display,
            {
                type Fields = Cons<
                    Field<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>, Name>,
                    Cons<Field<Symbol<3, Chars<'a', Chars<'g', Chars<'e', Nil>>>>, u8>, Nil>,
                >;
            }
            impl<Name> HasFieldsRef for Person<Name>
            where
                Name: Display,
            {
                type FieldsRef<'__a> = Cons<
                    Field<
                        Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        &'__a Name,
                    >,
                    Cons<Field<Symbol<3, Chars<'a', Chars<'g', Chars<'e', Nil>>>>, &'__a u8>, Nil>,
                >
                where
                    Self: '__a;
            }
            impl<Name> FromFields for Person<Name>
            where
                Name: Display,
            {
                fn from_fields(Cons(name, Cons(age, Nil)): Self::Fields) -> Self {
                    Self {
                        name: name.value,
                        age: age.value,
                    }
                }
            }
            impl<Name> ToFields for Person<Name>
            where
                Name: Display,
            {
                fn to_fields(self) -> Self::Fields {
                    Cons(self.name.into(), Cons(self.age.into(), Nil))
                }
            }
            impl<Name> ToFieldsRef for Person<Name>
            where
                Name: Display,
            {
                fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
                where
                    Self: '__a,
                {
                    Cons((&self.name).into(), Cons((&self.age).into(), Nil))
                }
            }
            ")
        }
    }

    #[test]
    fn test_generic_struct() {
        let name = "Alice".to_owned();

        let person1 = Person {
            name: name.clone(),
            age: 32,
        };

        let product = person1.clone().to_fields();
        assert_eq!(product, Cons(name.clone().into(), Cons(32.into(), Nil)));

        let product_ref = person1.to_fields_ref();
        assert_eq!(product_ref, Cons((&name).into(), Cons((&32).into(), Nil)));

        let person2 = Person::from_fields(product);

        assert_eq!(person1, person2);
    }
}

pub mod generic_lifetime_struct {
    use core::fmt::Display;

    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_derive_has_fields;

    snapshot_derive_has_fields! {
        #[derive(HasFields)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Person<'a, Name>
        where
            Name: Display,
        {
            pub name: &'a Name,
            pub age: &'a u8,
        }

        expand_person(output) {
            insta::assert_snapshot!(output, @"
            impl<'a, Name> HasFields for Person<'a, Name>
            where
                Name: Display,
            {
                type Fields = Cons<
                    Field<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>, &'a Name>,
                    Cons<Field<Symbol<3, Chars<'a', Chars<'g', Chars<'e', Nil>>>>, &'a u8>, Nil>,
                >;
            }
            impl<'a, Name> HasFieldsRef for Person<'a, Name>
            where
                Name: Display,
            {
                type FieldsRef<'__a> = Cons<
                    Field<
                        Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        &'__a &'a Name,
                    >,
                    Cons<
                        Field<Symbol<3, Chars<'a', Chars<'g', Chars<'e', Nil>>>>, &'__a &'a u8>,
                        Nil,
                    >,
                >
                where
                    Self: '__a;
            }
            impl<'a, Name> FromFields for Person<'a, Name>
            where
                Name: Display,
            {
                fn from_fields(Cons(name, Cons(age, Nil)): Self::Fields) -> Self {
                    Self {
                        name: name.value,
                        age: age.value,
                    }
                }
            }
            impl<'a, Name> ToFields for Person<'a, Name>
            where
                Name: Display,
            {
                fn to_fields(self) -> Self::Fields {
                    Cons(self.name.into(), Cons(self.age.into(), Nil))
                }
            }
            impl<'a, Name> ToFieldsRef for Person<'a, Name>
            where
                Name: Display,
            {
                fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
                where
                    Self: '__a,
                {
                    Cons((&self.name).into(), Cons((&self.age).into(), Nil))
                }
            }
            ")
        }
    }

    #[test]
    fn test_generic_lifetime_struct() {
        let name = "Alice".to_owned();

        let person1 = Person {
            name: &name,
            age: &32,
        };

        let product = person1.clone().to_fields();
        assert_eq!(product, Cons((&name).into(), Cons((&32).into(), Nil)));

        let product_ref = person1.to_fields_ref();
        assert_eq!(product_ref, Cons((&&name).into(), Cons((&&32).into(), Nil)));

        let person2 = Person::from_fields(product);

        assert_eq!(person1, person2);
    }
}

pub mod single_unnamed_field {
    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_derive_has_fields;

    snapshot_derive_has_fields! {
        #[derive(HasFields)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Person(String);

        expand_person(output) {
            insta::assert_snapshot!(output, @"
            impl HasFields for Person {
                type Fields = String;
            }
            impl HasFieldsRef for Person {
                type FieldsRef<'__a> = &'__a String where Self: '__a;
            }
            impl FromFields for Person {
                fn from_fields(field: Self::Fields) -> Self {
                    Self(field)
                }
            }
            impl ToFields for Person {
                fn to_fields(self) -> Self::Fields {
                    self.0
                }
            }
            impl ToFieldsRef for Person {
                fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
                where
                    Self: '__a,
                {
                    &self.0
                }
            }
            ")
        }
    }

    #[test]
    fn test_single_unnamed_field() {
        let name = "Alice".to_owned();

        let person1 = Person(name.clone());

        let product = person1.clone().to_fields();
        assert_eq!(product, name.clone());

        let product_ref = person1.to_fields_ref();
        assert_eq!(product_ref, &name);

        let person2 = Person::from_fields(product);

        assert_eq!(person1, person2);
    }
}

pub mod single_unnamed_multi_field {
    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_derive_has_fields;

    snapshot_derive_has_fields! {
        #[derive(HasFields)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Person(String, u8);

        expand_person(output) {
            insta::assert_snapshot!(output, @"
            impl HasFields for Person {
                type Fields = Cons<Field<Index<0>, String>, Cons<Field<Index<1>, u8>, Nil>>;
            }
            impl HasFieldsRef for Person {
                type FieldsRef<'__a> = Cons<
                    Field<Index<0>, &'__a String>,
                    Cons<Field<Index<1>, &'__a u8>, Nil>,
                >
                where
                    Self: '__a;
            }
            impl FromFields for Person {
                fn from_fields(Cons(field_1, Cons(field_0, Nil)): Self::Fields) -> Self {
                    Self(field_1.value, field_0.value)
                }
            }
            impl ToFields for Person {
                fn to_fields(self) -> Self::Fields {
                    Cons(self.0.into(), Cons(self.1.into(), Nil))
                }
            }
            impl ToFieldsRef for Person {
                fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
                where
                    Self: '__a,
                {
                    Cons((&self.0).into(), Cons((&self.1).into(), Nil))
                }
            }
            ")
        }
    }

    #[test]
    fn test_single_unnamed_multi_field() {
        let name = "Alice".to_owned();

        let person1 = Person(name.clone(), 32);

        let product = person1.clone().to_fields();
        assert_eq!(product, Cons(name.clone().into(), Cons(32.into(), Nil)));

        let product_ref = person1.to_fields_ref();
        assert_eq!(product_ref, Cons((&name).into(), Cons((&32).into(), Nil)));

        let person2 = Person::from_fields(product);

        assert_eq!(person1, person2);
    }
}
