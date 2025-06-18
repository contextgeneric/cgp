use cgp::prelude::*;

#[test]
fn test_simple_enum() {
    #[derive(Clone, Debug, Eq, PartialEq, HasFields)]
    pub enum Person {
        Anonymous(u32),
        Named(String),
    }

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

#[test]
fn test_generic_enum() {
    #[derive(Clone, Debug, Eq, PartialEq, HasFields)]
    pub enum Person<'a, Name> {
        Anonymous(u32),
        Named(&'a Name),
    }

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
