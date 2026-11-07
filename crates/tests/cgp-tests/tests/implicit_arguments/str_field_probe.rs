//! A `&str` implicit argument reads any `AsRef<str>` field.
//!
//! The argument lowers to a `HasField<Tag, Value: AsRef<str> + 'static>` bound and
//! reads the field with `.as_ref()`, so a `String`, a `&str`, or a `Cow<str>` field
//! all work, and none of them needs an allocator.
//!
//! See cgp-knowledge-base/cgp/reference/attributes/implicit.md.

use std::borrow::Cow;

use cgp::prelude::*;

#[cgp_fn]
pub fn greet_name(&self, #[implicit] name: &str) -> &str {
    name
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

#[test]
fn a_str_implicit_reads_every_as_ref_str_field() {
    let with_string = WithString {
        name: "a".to_owned(),
    };
    let with_str = WithStr { name: "b" };
    let with_cow = WithCow {
        name: Cow::Borrowed("c"),
    };

    assert_eq!(with_string.greet_name(), "a");
    assert_eq!(with_str.greet_name(), "b");
    assert_eq!(with_cow.greet_name(), "c");
}
