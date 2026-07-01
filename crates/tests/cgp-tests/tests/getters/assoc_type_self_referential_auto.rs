//! `#[cgp_auto_getter]` with a self-referential associated-type bound
//! (`Scalar: Mul<Output = Self::Scalar>`): the bound survives onto the generated
//! blanket impl with `Self::Scalar` rewritten to the inferred type parameter.
//!
//! See docs/reference/macros/cgp_auto_getter.md.

use core::ops::Mul;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasScalarType {
        type Scalar: Mul<Output = Self::Scalar> + Clone;

        fn scalar(&self) -> &Self::Scalar;
    }

    expand_has_scalar_type(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasScalarType {
            type Scalar: Mul<Output = Self::Scalar> + Clone;
            fn scalar(&self) -> &Self::Scalar;
        }
        impl<__Context__, Scalar> HasScalarType for __Context__
        where
            Scalar: Mul<Output = Scalar> + Clone,
            __Context__: HasField<
                Symbol<
                    6,
                    Chars<'s', Chars<'c', Chars<'a', Chars<'l', Chars<'a', Chars<'r', Nil>>>>>>,
                >,
                Value = Scalar,
            >,
        {
            type Scalar = Scalar;
            fn scalar(&self) -> &Self::Scalar {
                self.get_field(
                    ::core::marker::PhantomData::<
                        Symbol<
                            6,
                            Chars<
                                's',
                                Chars<'c', Chars<'a', Chars<'l', Chars<'a', Chars<'r', Nil>>>>>,
                            >,
                        >,
                    >,
                )
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub scalar: f64,
}

#[test]
fn test_auto_getter_scalar() {
    let app = App { scalar: 2.0 };

    assert_eq!(*app.scalar(), 2.0);
}
