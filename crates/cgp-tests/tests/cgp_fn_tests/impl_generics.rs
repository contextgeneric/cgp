use core::fmt::Display;

use cgp::prelude::*;

#[cgp_fn]
#[impl_generics(Name: Display)]
pub fn greet(&self, #[implicit] name: &Name) -> String
where
    Name: Display,
{
    format!("Hello, {}!", name)
}

#[cgp_fn]
#[uses(Greet)]
pub fn test_greet(&self) {
    assert_eq!(self.greet(), "Hello, John!");
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

#[test]
fn test_impl_generics() {
    let person = Person {
        name: "John".to_string(),
    };

    person.test_greet();
}
