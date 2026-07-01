//! `#[cgp_auto_getter]` returning `&[u8]`: the blanket impl reads a field whose
//! type is `AsRef<[u8]> + 'static` (e.g. `Vec<u8>`) named after the method and
//! calls `.as_ref()`, exposing an owned byte buffer as a slice without wiring.
//!
//! See docs/reference/macros/cgp_auto_getter.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasFoo {
        fn foo(&self) -> &[u8];
    }

    expand_has_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFoo {
            fn foo(&self) -> &[u8];
        }
        impl<__Context__> HasFoo for __Context__
        where
            __Context__: HasField<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value: AsRef<[u8]> + 'static,
            >,
        {
            fn foo(&self) -> &[u8] {
                self.get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                        >,
                    )
                    .as_ref()
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub foo: Vec<u8>,
}

#[test]
pub fn test_slice_auto_getter() {
    let context = App { foo: vec![1, 2, 3] };

    assert_eq!(context.foo(), &[1, 2, 3]);
}
