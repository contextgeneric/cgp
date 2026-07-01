//! Runtime behavior of the `Symbol!` type-level string tag: a `Symbol!` value
//! `Display`s as the string it encodes, and its `StaticString::VALUE` recovers
//! the original literal — including the empty string and multi-byte Unicode.
//!
//! See docs/reference/macros/symbol.md and docs/reference/traits/has_field.md.

use cgp::core::field::traits::StaticString;
use cgp::prelude::*;

#[test]
pub fn test_symbol_display() {
    let val = <Symbol!("hello")>::default();
    assert_eq!(val.to_string(), "hello");
}

#[test]
fn test_static_chars() {
    assert_eq!(<Symbol!("") as StaticString>::VALUE, "");
    assert_eq!(<Symbol!("a") as StaticString>::VALUE, "a");
    assert_eq!(<Symbol!("abc") as StaticString>::VALUE, "abc");
    assert_eq!(<Symbol!("世界你好") as StaticString>::VALUE, "世界你好");
    assert_eq!(
        <Symbol!("a quick brown fox jumps over the lazy dog") as StaticString>::VALUE,
        "a quick brown fox jumps over the lazy dog",
    );
}
