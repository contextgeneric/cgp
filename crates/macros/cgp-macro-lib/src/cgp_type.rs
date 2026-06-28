use cgp_macro_core::types::cgp_component::{CgpComponentRawArgs, ItemCgpComponent};
use cgp_macro_core::types::cgp_type::{ItemCgpType, extract_item_type_from_trait};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemTrait, parse2};

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

    let item_cgp_type = ItemCgpType {
        item_component: evaluated,
    };

    let items = item_cgp_type.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
