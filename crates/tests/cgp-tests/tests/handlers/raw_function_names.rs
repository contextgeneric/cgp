//! `#[cgp_computer]` and `#[cgp_producer]` on functions named with a raw
//! identifier.
//!
//! The default provider name is the function name in PascalCase, read without
//! the `r#` prefix, which cannot begin a longer identifier: `r#type` names the
//! provider `Type`, and `r#loop` names it `Loop`.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_computer.md and
//! cgp-knowledge-base/cgp/implementation/entrypoints/cgp_producer.md.

use cgp::extra::handler::{Computer, Producer};
use cgp::prelude::*;

#[cgp_computer]
fn r#type(value: u64) -> u64 {
    value + 1
}

#[cgp_producer]
fn r#loop() -> u64 {
    42
}

pub struct App;

#[test]
fn test_raw_function_names() {
    assert_eq!(Type::compute(&App, PhantomData::<()>, 1), 2);
    assert_eq!(Loop::produce(&App, PhantomData::<()>), 42);
}
