use alloc::format;

use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{parse_quote, Ident, ItemTrait};

use crate::derive_component::derive_component_with_ast;
use crate::parse::{ComponentSpec, Entries};
use crate::type_component::{derive_type_alias, derive_type_providers, extract_item_type};

pub fn cgp_type(attrs: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let Entries { mut entries } = syn::parse2(attrs)?;

    let consumer_trait: ItemTrait = syn::parse2(body)?;

    let item_type = extract_item_type(&consumer_trait)?.clone();

    entries.entry("provider".into()).or_insert_with(|| {
        let provider_name = Ident::new(
            &format!("{}TypeProvider", item_type.ident),
            item_type.ident.span(),
        );
        parse_quote!( #provider_name )
    });

    let spec = ComponentSpec::from_entries(&entries)?;

    let component = derive_component_with_ast(&spec, consumer_trait)?;

    let alias_type = derive_type_alias(&component.consumer_trait, &spec.context_type, &item_type)?;

    let type_provider_impls = derive_type_providers(&spec, &component.provider_trait, &item_type)?;

    let mut out = quote! {
        #component

        #alias_type
    };

    out.append_all(type_provider_impls);

    Ok(out)
}
