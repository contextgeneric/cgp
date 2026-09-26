//! `WrapError` prints its detail alone with `{}` and returns the wrapped error as its `source`, so
//! walking the chain prints each message exactly once; `{:#}` and `{:?}` print the whole chain.
//! `StringError` prints its message unquoted for both `{}` and `{:?}`.
//!
//! See cgp-knowledge-base/projects/error/cgp-error-std/testing.md.

use core::error::Error as _;

use cgp_error_std::{Error, StringError, WrapError};

#[test]
fn test_wrap_error_chain() {
    let inner: Error = Box::new(StringError::from(String::from("inner")));
    let middle: Error = Box::new(WrapError {
        detail: String::from("middle"),
        source: inner,
    });
    let outer = WrapError {
        detail: String::from("outer"),
        source: middle,
    };

    let mut messages = Vec::new();
    let mut current: Option<&(dyn core::error::Error + 'static)> = Some(&outer);
    while let Some(error) = current {
        messages.push(error.to_string());
        current = error.source();
    }
    assert_eq!(messages, ["outer", "middle", "inner"]);

    assert_eq!(format!("{outer}"), "outer");
    assert_eq!(format!("{outer:#}"), "outer: middle: inner");
    assert_eq!(format!("{outer:?}"), "outer: middle: inner");
    assert!(outer.source().is_some());
}

#[test]
fn test_string_error_format() {
    let error = StringError::from(String::from("message"));
    assert_eq!(format!("{error}"), "message");
    assert_eq!(format!("{error:?}"), "message");
}
