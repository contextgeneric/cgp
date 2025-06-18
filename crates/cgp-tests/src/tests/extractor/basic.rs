use cgp::prelude::*;

#[derive(ExtractField)]
pub enum Context {
    Foo(u64),
    Bar(String),
    Baz(bool),
}
