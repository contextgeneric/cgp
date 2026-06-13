use cgp_macro_core::types::attributes::CgpComponentAttributes;
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

    let mut item_trait: ItemTrait = syn::parse2(body)?;

    let attributes = CgpComponentAttributes::parse(&mut item_trait.attrs)?;

    item_trait.supertraits.extend(attributes.extend.clone());

    attributes.use_type.transform_item_trait(&mut item_trait)?;

    let context_type = Ident::new("__Context__", Span::call_site());

    let (fields, field_type) = parse_getter_fields(&context_type, &item_trait)?;

    let blanket_impl = derive_blanket_impl(&context_type, &item_trait, &fields, &field_type)?;

    Ok(quote! {
        #item_trait
        #blanket_impl
    })
}
