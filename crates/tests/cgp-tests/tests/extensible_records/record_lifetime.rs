//! `#[derive(CgpData)]` on a *struct with a lifetime parameter*, exercising the
//! full builder path. `generic_record` covers a record whose parameters are
//! types; this covers the lifetime case, where the struct's `'a` is threaded
//! through the `__Partial…` companion (whose fields are `<F as MapType>::Map<&'a
//! T>`) and every builder impl (`HasBuilder`, `BuildField`, `FinalizeBuild`).
//! It is the record-side counterpart of `derive_cgp_data_lifetime` on enums.
//!
//! A plain (non-snapshot) test: the derive's expansion shape is already pinned
//! by `record_derive` and `struct_generic_lifetime`, so this only guards that
//! the builder round-trips at compile and run time on a borrowed record.
//!
//! See docs/reference/derives/derive_cgp_data.md.

use core::marker::PhantomData;

use cgp::prelude::*;

#[derive(Debug, Eq, PartialEq, CgpData)]
pub struct Borrowed<'a> {
    pub name: &'a str,
    pub count: u32,
}

#[test]
fn test_lifetime_record_builder() {
    let owner = "alice".to_owned();

    // Build the borrowed record field by field, out of declaration order, to
    // confirm the partial companion tracks presence per field with `'a` bound in.
    let built: Borrowed = Borrowed::builder()
        .build_field(PhantomData::<Symbol!("count")>, 3)
        .build_field(PhantomData::<Symbol!("name")>, owner.as_str())
        .finalize_build();

    assert_eq!(
        built,
        Borrowed {
            name: "alice",
            count: 3
        }
    );

    // `into_builder` turns a complete borrowed record back into an all-present
    // partial, and `finalize_build` reconstructs it.
    let round_tripped: Borrowed = built.into_builder().finalize_build();
    assert_eq!(
        round_tripped,
        Borrowed {
            name: "alice",
            count: 3
        }
    );
}
