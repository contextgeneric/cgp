//! `#[cgp_fn]` with an immutable `&[T]` `#[implicit]` argument under a `&mut self`
//! receiver.
//!
//! A `&[T]` implicit reads any `AsRef<[T]>` field (e.g. `Vec<u8>`) and calls
//! `.as_ref()`. The access mode follows the *argument's* own type, not the
//! receiver's, so the immutable slice reads through `HasField`/`get_field` with an
//! `AsRef<[u8]>` bound even though the receiver is `&mut self` — it is not forced
//! into a plain-reference `Value = [u8]` bound that no context could satisfy.
//!
//! See docs/reference/macros/cgp_fn.md and docs/reference/attributes/implicit.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    pub fn count_matches(&mut self, #[implicit] data: &[u8], byte: u8) -> usize {
        data.iter().filter(|b| **b == byte).count()
    }

    expand_count_matches(output) {
        insta::assert_snapshot!(output, @"
        pub trait CountMatches {
            fn count_matches(&mut self, byte: u8) -> usize;
        }
        impl<__Context__> CountMatches for __Context__
        where
            Self: HasField<
                Symbol<4, Chars<'d', Chars<'a', Chars<'t', Chars<'a', Nil>>>>>,
                Value: AsRef<[u8]> + 'static,
            >,
        {
            fn count_matches(&mut self, byte: u8) -> usize {
                let data: &[u8] = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'d', Chars<'a', Chars<'t', Chars<'a', Nil>>>>>,
                        >,
                    )
                    .as_ref();
                data.iter().filter(|b| **b == byte).count()
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
fn test_count_matches() {
    let mut buffer = Buffer {
        data: vec![1, 2, 2, 3, 2],
    };

    assert_eq!(buffer.count_matches(2), 3);
}
