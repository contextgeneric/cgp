use alloc::format;
use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, ItemTrait, parse_quote, parse2};

use crate::derive_component::derive_component_with_ast;
use crate::parse::{ComponentSpec, Entries};
use crate::type_component::{derive_type_providers, extract_item_type_from_trait};

pub fn cgp_type(attrs: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let mut entries = if let Ok(provider_ident) = parse2::<Ident>(attrs.clone()) {
        BTreeMap::from([("provider".to_owned(), provider_ident.to_token_stream())])
    } else {
        parse2::<Entries>(attrs)?.entries
    };

    let consumer_trait: ItemTrait = syn::parse2(body)?;

    let item_type = extract_item_type_from_trait(&consumer_trait)?.clone();

    entries.entry("provider".into()).or_insert_with(|| {
        let provider_name = Ident::new(
            &format!("{}TypeProvider", item_type.ident),
            item_type.ident.span(),
        );
        parse_quote!( #provider_name )
    });

    let spec = ComponentSpec::from_entries(&entries)?;

    let component = derive_component_with_ast(&spec, consumer_trait)?;

    let type_provider_impls = derive_type_providers(&spec, &component.provider_trait, &item_type)?;

    let out = quote! {
        #component

        #(#type_provider_impls)*
    };

    Ok(out)
}
