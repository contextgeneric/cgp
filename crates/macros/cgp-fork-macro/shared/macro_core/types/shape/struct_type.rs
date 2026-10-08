use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{Fields, Type};

use crate::macro_core::functions::parse_shape_fields;
use crate::macro_core::types::cgp_data::item_fields_to_product_type;

/// The `Struct!` macro: the body of a named or tuple struct, held as the `syn::Fields` the
/// struct declaration would have.
///
/// Holding the derive's own input form lets [`eval`](Self::eval) run the `#[derive(HasFields)]`
/// encoder unchanged, so `Struct! { … }` is by construction the type a derived `Fields` is.
pub struct StructType {
    pub fields: Fields,
}

impl StructType {
    /// Encode the fields as the derive does: a product of `Field` entries, keyed by `Symbol!`
    /// for named fields and by `Index<N>` for positional ones, with a one-field tuple body
    /// encoding to its bare type and an empty body to `Nil`.
    pub fn eval(&self) -> syn::Result<Type> {
        item_fields_to_product_type(&self.fields, &TokenStream::new())
    }
}

impl Parse for StructType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fields = parse_shape_fields(input)?;

        Ok(Self { fields })
    }
}
