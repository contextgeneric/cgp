//! `#[cgp_auto_getter]` with a `&mut self` receiver returning `&mut [u32]`: the
//! blanket impl reads a field whose type is `AsMut<[u32]> + 'static` (e.g.
//! `Vec<u32>`) mutably through `get_field_mut` and calls `.as_mut()`, the mutable
//! mirror of the shared `&[T]` slice getter.
//!
//! See docs/reference/macros/cgp_auto_getter.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasFoo {
        fn foo(&mut self) -> &mut [u32];
    }

    expand_has_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFoo {
            fn foo(&mut self) -> &mut [u32];
        }
        impl<__Context__> HasFoo for __Context__
        where
            __Context__: HasFieldMut<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value: AsMut<[u32]> + 'static,
            >,
        {
            fn foo(&mut self) -> &mut [u32] {
                self.get_field_mut(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                        >,
                    )
                    .as_mut()
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub foo: Vec<u32>,
}

#[test]
pub fn test_mut_slice_auto_getter() {
    let mut context = App { foo: vec![1, 2, 3] };

    context.foo()[0] = 10;

    assert_eq!(context.foo, vec![10, 2, 3]);
}
