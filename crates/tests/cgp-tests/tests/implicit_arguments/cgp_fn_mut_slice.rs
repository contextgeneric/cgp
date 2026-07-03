//! `#[cgp_fn]` with a mutable `&mut [T]` `#[implicit]` argument under a `&mut self`
//! receiver.
//!
//! A `&mut [T]` implicit reads any `AsMut<[T]>` field (e.g. `Vec<u8>`) through
//! `HasFieldMut`/`get_field_mut` and calls `.as_mut()`, the mutable mirror of the
//! shared `&[T]` slice's `AsRef<[T]>`/`.as_ref()` read. The access mode follows the
//! *argument's* own type, so it is the `&mut` in `&mut [T]` — not the receiver —
//! that selects the mutable read.
//!
//! See docs/reference/macros/cgp_fn.md and docs/reference/attributes/implicit.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    pub fn zero_all(&mut self, #[implicit] data: &mut [u8]) {
        for byte in data.iter_mut() {
            *byte = 0;
        }
    }

    expand_zero_all(output) {
        insta::assert_snapshot!(output, @"
        pub trait ZeroAll {
            fn zero_all(&mut self);
        }
        impl<__Context__> ZeroAll for __Context__
        where
            Self: HasFieldMut<
                Symbol<4, Chars<'d', Chars<'a', Chars<'t', Chars<'a', Nil>>>>>,
                Value: AsMut<[u8]> + 'static,
            >,
        {
            fn zero_all(&mut self) {
                let data: &mut [u8] = self
                    .get_field_mut(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'d', Chars<'a', Chars<'t', Chars<'a', Nil>>>>>,
                        >,
                    )
                    .as_mut();
                for byte in data.iter_mut() {
                    *byte = 0;
                }
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct Buffer {
    pub data: Vec<u8>,
}

#[test]
fn test_zero_all() {
    let mut buffer = Buffer {
        data: vec![1, 2, 3],
    };

    buffer.zero_all();

    assert_eq!(buffer.data, vec![0, 0, 0]);
}
