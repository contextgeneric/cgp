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

#[cgp_fn(CanCalculateRectangleArea)]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub trait CheckRectangle: CanCalculateRectangleArea {}
impl CheckRectangle for Rectangle {}
