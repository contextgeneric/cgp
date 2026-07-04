//! `#[derive(CgpData)]` on an enum with a lifetime parameter named `'a`. The
//! extractor codegen introduces its own lifetime for the borrowed extractor
//! (`ExtractorRef`/`ExtractorMut`) and the borrowed `FinalizeExtract` impl; that
//! lifetime is the reserved `'__a__`, so it does not collide with a user enum
//! whose own lifetime happens to be named `'a`. This file guards against a
//! regression where a bare `'a` was emitted and clashed with the user's `'a`.
//!
//! A plain (non-snapshot) test: the concept's snapshots already pin the
//! extractor expansion shape, so this only exercises the corner case at compile
//! and run time.
//!
//! See docs/reference/derives/derive_cgp_data.md and
//! docs/reference/derives/derive_extract_field.md.

use core::marker::PhantomData;

use cgp::core::field::traits::FinalizeExtractResult;
use cgp::prelude::*;

#[derive(Debug, Eq, PartialEq, CgpData)]
pub enum Message<'a> {
    Text(&'a str),
    Code(u32),
}

#[test]
fn test_lifetime_enum_extractor() {
    // Owned extraction: chain `extract_field` down to the empty remainder.
    let code = Message::Code(7);
    let value = match code
        .to_extractor()
        .extract_field(PhantomData::<Symbol!("Text")>)
    {
        Ok(_text) => 0,
        Err(remainder) => remainder
            .extract_field(PhantomData::<Symbol!("Code")>)
            .finalize_extract_result(),
    };
    assert_eq!(value, 7);

    // Borrowed extraction through `ExtractorRef` (introduces the `'__a__`
    // lifetime that must not collide with the enum's own `'a`).
    let hello = "hello".to_owned();
    let text = Message::Text(&hello);
    match text
        .extractor_ref()
        .extract_field(PhantomData::<Symbol!("Text")>)
    {
        Ok(borrowed) => assert_eq!(*borrowed, "hello"),
        Err(_) => panic!("expected the Text variant"),
    }

    // Mutable borrowed extraction through `ExtractorMut`.
    let mut number = Message::Code(1);
    if let Ok(value) = number
        .extractor_mut()
        .extract_field(PhantomData::<Symbol!("Code")>)
    {
        *value += 41;
    }
    assert_eq!(number, Message::Code(42));
}
