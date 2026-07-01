//! `#[derive(HasField)]` on a tuple struct: positional fields have no string
//! name, so the derive keys each one by the type-level number `Index<N>`
//! (`Index<0>` for `.0`, `Index<1>` for `.1`) instead of a `Symbol!`.
//!
//! See docs/reference/derives/derive_has_field.md and
//! docs/reference/types/index.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_field;

snapshot_derive_has_field! {
    #[derive(HasField)]
    pub struct Context(pub String, pub u64);

    expand_context(output) {
        insta::assert_snapshot!(output, @"
        impl HasField<Index<0>> for Context {
            type Value = String;
            fn get_field(&self, key: ::core::marker::PhantomData<Index<0>>) -> &Self::Value {
                &self.0
            }
        }
        impl HasFieldMut<Index<0>> for Context {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Index<0>>,
            ) -> &mut Self::Value {
                &mut self.0
            }
        }
        impl HasField<Index<1>> for Context {
            type Value = u64;
            fn get_field(&self, key: ::core::marker::PhantomData<Index<1>>) -> &Self::Value {
                &self.1
            }
        }
        impl HasFieldMut<Index<1>> for Context {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Index<1>>,
            ) -> &mut Self::Value {
                &mut self.1
            }
        }
        ")
    }
}

pub trait CheckHasFieldImpls:
    HasField<Index<0>, Value = String> + HasField<Index<1>, Value = u64>
{
}

impl CheckHasFieldImpls for Context {}

#[test]
fn test_has_field_index() {
    let context = Context("test".to_owned(), 1);
    assert_eq!(context.get_field(PhantomData::<Index<0>>), &"test");
    assert_eq!(context.get_field(PhantomData::<Index<1>>), &1);
}
