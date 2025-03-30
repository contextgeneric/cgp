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
        assert_eq!(person_a2, Either::Left(Cons(42.into(), Nil).into()));

        let person_a3 = Person::from_fields(person_a2);
        assert_eq!(person_a3, person_a1);
    }

    {
        let person_b1 = Person::Named("Alice".to_owned());

        let person_b2 = person_b1.clone().to_fields();
        assert_eq!(
            person_b2,
            Either::Right(Either::Left(Cons("Alice".to_owned().into(), Nil).into()))
        );

        let person_b3 = Person::from_fields(person_b2);
        assert_eq!(person_b3, person_b1);
    }
}
