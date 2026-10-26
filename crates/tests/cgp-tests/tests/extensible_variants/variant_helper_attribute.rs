//! A variant helper attribute of another derive, here `Default`'s `#[default]`, on an enum that also
//! derives `CgpData`.
//!
//! The extractor's companion enums copy the input's variants, but keep only their documentation,
//! so the `#[default]` stays on `Mode` alone and does not reach a companion that lacks
//! `#[derive(Default)]`.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/derive_extract_field.md.

use core::marker::PhantomData;

use cgp::prelude::*;

#[derive(Debug, Default, PartialEq, CgpData)]
pub enum Mode {
    #[default]
    Off,
    On(u64),
}

#[test]
fn test_variant_helper_attribute() {
    assert_eq!(Mode::default(), Mode::Off);

    let extracted = Mode::On(3)
        .to_extractor()
        .extract_field(PhantomData::<Symbol!("On")>);
    assert_eq!(extracted.ok(), Some(3));
}
