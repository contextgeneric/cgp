use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Ident, ItemTrait};

use crate::derive_getter::{derive_blanket_impl, parse_getter_fields};

pub fn cgp_auto_getter(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "#[derive_auto_getter] does not accept any attribute argument",
        ));
    }

    let consumer_trait: ItemTrait = syn::parse2(body)?;

    let context_type = Ident::new("__Context__", Span::call_site());

    let (fields, field_type) = parse_getter_fields(&context_type, &consumer_trait)?;

    let blanket_impl = derive_blanket_impl(&context_type, &consumer_trait, &fields, &field_type)?;

    Ok(quote! {
        #consumer_trait
        #blanket_impl
    })
}
