use cgp::prelude::*;

#[cgp_fn]
pub fn greet(&self, #[implicit] name: &str) {
    println!("Hello, {}!", name);
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

pub trait CheckPerson: Greet {}
impl CheckPerson for Person {}
