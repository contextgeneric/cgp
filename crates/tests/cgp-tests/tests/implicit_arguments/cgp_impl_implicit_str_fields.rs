//! A `#[cgp_impl]` `&str` implicit reads every `AsRef<str>` field.
//!
//! The snapshot tests pin the expansion; this one runs the provider against a
//! `String`, a `&str`, and a `Cow<str>` field, so the bound is exercised at runtime
//! as well as at compile time.
//!
//! See cgp-knowledge-base/cgp/reference/attributes/implicit.md.

use std::borrow::Cow;

use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

#[cgp_impl(new GreetHello)]
impl Greeter {
    fn greet(&self, #[implicit] name: &str) -> String {
        format!("Hello, {name}!")
    }
}

#[derive(HasField)]
pub struct WithString {
    pub name: String,
}

#[derive(HasField)]
pub struct WithStr<'a> {
    pub name: &'a str,
}

#[derive(HasField)]
pub struct WithCow<'a> {
    pub name: Cow<'a, str>,
}

delegate_components! {
    WithString {
        GreeterComponent: GreetHello,
    }
}

delegate_components! {
    <'a> WithStr<'a> {
        GreeterComponent: GreetHello,
    }
}

delegate_components! {
    <'a> WithCow<'a> {
        GreeterComponent: GreetHello,
    }
}

check_components! {
    WithString {
        GreeterComponent,
    }
}

#[test]
fn a_str_implicit_reads_every_as_ref_str_field() {
    let with_string = WithString {
        name: "a".to_owned(),
    };
    let with_str = WithStr { name: "b" };
    let with_cow = WithCow {
        name: Cow::Borrowed("c"),
    };

    assert_eq!(with_string.greet(), "Hello, a!");
    assert_eq!(with_str.greet(), "Hello, b!");
    assert_eq!(with_cow.greet(), "Hello, c!");
}
