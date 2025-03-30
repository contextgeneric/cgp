use cgp::prelude::*;

#[test]
fn test_simple_enum() {
    #[derive(HasFields)]
    pub enum Person {
        Anonymous(u32),
        Named(String),
    }

    let person_a1 = Person::Anonymous(42);

    let person_b1 = Person::Named("Alice".to_owned());
}
