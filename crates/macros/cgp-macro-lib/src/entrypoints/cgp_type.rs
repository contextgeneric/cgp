use alloc::format;

use cgp_macro_core::types::cgp_component::{CgpComponentRawArgs, ItemCgpComponent};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemTrait, parse2};

use crate::type_component::{derive_type_providers, extract_item_type_from_trait};

pub fn cgp_type(attrs: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let mut raw_args: CgpComponentRawArgs = parse2(attrs.clone())?;

    let item_trait: ItemTrait = syn::parse2(body)?;

    let item_type = extract_item_type_from_trait(&item_trait)?.clone();

    if raw_args.provider_ident.is_none() {
        raw_args.provider_ident = Some(Ident::new(
            &format!("{}TypeProvider", item_type.ident),
            item_type.ident.span(),
        ));
    }

    let item_cgp_component = ItemCgpComponent {
        args: raw_args.try_into()?,
        item_trait,
    };

    let evaluated = item_cgp_component.preprocess()?.eval()?;

    let items = evaluated.to_items()?;

    let type_provider_impls =
        derive_type_providers(&evaluated.args, &evaluated.provider_trait, &item_type)?;

    let out = quote! {
        #( #items )*

        #(#type_provider_impls)*
    };

    Ok(out)
}
