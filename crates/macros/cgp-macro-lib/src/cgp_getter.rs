use cgp_macro_core::types::cgp_component::{CgpComponentRawArgs, ItemCgpComponent};
use cgp_macro_core::types::cgp_getter::ItemCgpGetter;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemTrait, TraitItem, parse2};

use crate::cgp_type;

pub fn cgp_getter(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let item_trait: ItemTrait = syn::parse2(body.clone())?;

    // A getter that is only an associated type is an abstract type. Reuse the
    // #[cgp_type] provider path (UseType / WithProvider) rather than a field read.
    if is_assoc_type_only(&item_trait) {
        return cgp_type(attr, body);
    }

    let mut raw_args: CgpComponentRawArgs = parse2(attr.clone())?;

    if raw_args.provider_ident.is_none()
        && let Some(field_name) = item_trait.ident.to_string().strip_prefix("Has")
        && !field_name.is_empty()
    {
        raw_args.provider_ident = Some(Ident::new(
            &format!("{field_name}Getter"),
            item_trait.ident.span(),
        ));
    }

    let item_cgp_component = ItemCgpComponent {
        args: raw_args.try_into()?,
        item_trait,
    };

    let evaluated = item_cgp_component.preprocess()?.eval()?;

    let item_getter = ItemCgpGetter::try_from(evaluated)?;

    let items = item_getter.to_items()?;

    let derived = quote! {
        #( #items )*
    };

    Ok(derived)
}

fn is_assoc_type_only(item_trait: &ItemTrait) -> bool {
    matches!(item_trait.items.as_slice(), [TraitItem::Type(_)])
}
