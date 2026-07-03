//! `#[cgp_fn]` with an owned, non-path `#[implicit]` argument (a tuple).
//!
//! Owned implicit arguments are read by value and `.clone()`d regardless of their
//! shape — a tuple, an array, or a plain path type all read a `Value = T` field.
//! This pins that a tuple field is accepted rather than rejected.
//!
//! See docs/reference/macros/cgp_fn.md and docs/reference/attributes/implicit.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    pub fn origin_distance(&self, #[implicit] point: (f64, f64)) -> f64 {
        (point.0 * point.0 + point.1 * point.1).sqrt()
    }

    expand_origin_distance(output) {
        insta::assert_snapshot!(output, @"
        pub trait OriginDistance {
            fn origin_distance(&self) -> f64;
        }
        impl<__Context__> OriginDistance for __Context__
        where
            Self: HasField<
                Symbol<5, Chars<'p', Chars<'o', Chars<'i', Chars<'n', Chars<'t', Nil>>>>>>,
                Value = (f64, f64),
            >,
        {
            fn origin_distance(&self) -> f64 {
                let point: (f64, f64) = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'p', Chars<'o', Chars<'i', Chars<'n', Chars<'t', Nil>>>>>,
                            >,
                        >,
                    )
                    .clone();
                (point.0 * point.0 + point.1 * point.1).sqrt()
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct Location {
    pub point: (f64, f64),
}

#[test]
fn test_origin_distance() {
    let location = Location { point: (3.0, 4.0) };

    assert_eq!(location.origin_distance(), 5.0);
}
