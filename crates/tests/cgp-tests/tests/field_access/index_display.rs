//! Runtime behavior of the `Index<N>` type-level number tag: an `Index<N>`
//! value `Display`s as its underlying number, with no Greek-letter alias.
//!
//! See docs/reference/types/index.md.

use cgp::prelude::*;

#[test]
pub fn test_index_display() {
    let val: Index<123> = Default::default();
    assert_eq!(val.to_string(), "123");
}
