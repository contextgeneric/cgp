//! `#[cgp_fn]` with a mutable `Option<&mut T>` `#[implicit]` argument under a
//! `&mut self` receiver.
//!
//! An `Option<&mut T>` implicit reads an `Option<T>` field through
//! `HasFieldMut`/`get_field_mut` and calls `.as_mut()`, yielding `Option<&mut T>` —
//! the mutable mirror of the shared `Option<&T>`'s `.as_ref()` read. The read
//! borrows the field with the *inner* reference's mutability, so the `&mut` inside
//! the `Option` selects the mutable read and requires the `&mut self` receiver.
//!
//! See docs/reference/macros/cgp_fn.md and docs/reference/attributes/implicit.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    pub fn clear_slot(&mut self, #[implicit] slot: Option<&mut u8>) {
        if let Some(value) = slot {
            *value = 0;
        }
    }

    expand_clear_slot(output) {
        insta::assert_snapshot!(output, @"
        pub trait ClearSlot {
            fn clear_slot(&mut self);
        }
        impl<__Context__> ClearSlot for __Context__
        where
            Self: HasFieldMut<
                Symbol<4, Chars<'s', Chars<'l', Chars<'o', Chars<'t', Nil>>>>>,
                Value = Option<u8>,
            >,
        {
            fn clear_slot(&mut self) {
                let slot: Option<&mut u8> = self
                    .get_field_mut(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'s', Chars<'l', Chars<'o', Chars<'t', Nil>>>>>,
                        >,
                    )
                    .as_mut();
                if let Some(value) = slot {
                    *value = 0;
                }
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct Register {
    pub slot: Option<u8>,
}

#[test]
fn test_clear_slot() {
    let mut register = Register { slot: Some(42) };

    register.clear_slot();

    assert_eq!(register.slot, Some(0));
}
