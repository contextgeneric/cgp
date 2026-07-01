use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::token::Eq;
use syn::{Error, ImplItem, ImplItemConst, TraitItem, Type, Visibility};

use crate::functions::{
    parse_internal, signature_to_delegated_impl_item_fn, trait_to_impl_item_type,
};

/// Build impl items that forward each trait item (method, associated type, or
/// const) to `delegate_type`, projecting types/consts through `provider_trait_path`.
pub fn trait_items_to_delegated_impl_items(
    trait_items: &[TraitItem],
    delegate_type: &Type,
    provider_trait_path: &Type,
) -> syn::Result<Vec<ImplItem>> {
    trait_items
        .iter()
        .map(|trait_item| {
            trait_item_to_delegated_impl_items(trait_item, delegate_type, provider_trait_path)
        })
        .collect()
}

pub fn trait_item_to_delegated_impl_items(
    trait_item: &TraitItem,
    delegate_type: &Type,
    provider_trait_path: &Type,
) -> syn::Result<ImplItem> {
    let impl_item = match trait_item {
        TraitItem::Fn(trait_fn) => {
            let impl_fn = signature_to_delegated_impl_item_fn(&trait_fn.sig, delegate_type)?;

            ImplItem::Fn(impl_fn)
        }
        TraitItem::Type(trait_type) => {
            let type_name = &trait_type.ident;
            let type_generics = trait_type.generics.split_for_impl().1;
            let delegate_type = parse_internal! {
                < #delegate_type as #provider_trait_path > :: #type_name #type_generics
            };

            let impl_type = trait_to_impl_item_type(trait_type, delegate_type);

            ImplItem::Type(impl_type)
        }
        TraitItem::Const(trait_item_const) => {
            let const_ident = &trait_item_const.ident;
            let type_generics = trait_item_const.generics.split_for_impl().1;

            let impl_expr = parse_internal! {
                < #delegate_type as #provider_trait_path > :: #const_ident #type_generics
            };

            let impl_item_const = ImplItemConst {
                attrs: trait_item_const.attrs.clone(),
                vis: Visibility::Inherited,
                defaultness: None,
                const_token: trait_item_const.const_token,
                ident: trait_item_const.ident.clone(),
                generics: trait_item_const.generics.clone(),
                colon_token: trait_item_const.colon_token,
                ty: trait_item_const.ty.clone(),
                eq_token: Eq(Span::call_site()),
                expr: impl_expr,
                semi_token: trait_item_const.semi_token,
            };

            ImplItem::Const(impl_item_const)
        }
        _ => {
            return Err(Error::new(
                trait_item.span(),
                format!("unsupported trait item: {trait_item:?}"),
            ));
        }
    };

    Ok(impl_item)
}
