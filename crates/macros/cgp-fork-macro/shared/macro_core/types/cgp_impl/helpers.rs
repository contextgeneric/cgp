use quote::ToTokens;
use syn::{
    Attribute, Error, Ident, ImplItem, ImplItemFn, Item, ItemImpl, ItemTrait, Meta, TraitItemFn,
    Type,
};

use crate::macro_core::functions::parse_internal;
use crate::macro_core::types::cgp_auto_impl::derive_blanket_items;
use crate::macro_core::types::ident::{IdentWithTypeArgs, PathWithTypeArgs};

/// Pull `#[helper]` methods out of a `#[cgp_impl]` block and turn them into a
/// private blanket trait via [`derive_blanket_items`].
///
/// Helpers are not provider-trait methods, so they are removed before
/// `replace_self` rewrites the remaining methods into provider form. The helper
/// bodies stay in consumer form (`self` / `&self`) on the blanket impl.
pub fn extract_helper_items(
    item_impl: &mut ItemImpl,
    context_type: &Type,
    provider_type: &Type,
    provider_trait_path: &PathWithTypeArgs,
) -> syn::Result<Vec<Item>> {
    let helpers = take_helper_fns(&mut item_impl.items)?;

    if helpers.is_empty() {
        return Ok(Vec::new());
    }

    let trait_ident = helper_trait_ident(provider_type, provider_trait_path)?;

    let helper_trait = helper_trait(&trait_ident, &helpers)?;

    let (emitted_trait, helper_impl) =
        derive_blanket_items(context_type, &helper_trait, item_impl.generics.clone())?;

    Ok(vec![emitted_trait.into(), helper_impl.into()])
}

fn helper_trait(trait_ident: &Ident, helpers: &[ImplItemFn]) -> syn::Result<ItemTrait> {
    let mut items: Vec<TraitItemFn> = Vec::new();

    for func in helpers {
        let attrs = &func.attrs;
        let sig = &func.sig;
        let block = &func.block;
        items.push(parse_internal! {
            #( #attrs )*
            #sig #block
        });
    }

    let item_trait: ItemTrait = parse_internal! {
        #[allow(non_camel_case_types)]
        trait #trait_ident {
            #( #items )*
        }
    };

    Ok(item_trait)
}

fn helper_trait_ident(
    provider_type: &Type,
    provider_trait_path: &PathWithTypeArgs,
) -> syn::Result<Ident> {
    let trait_ident = provider_trait_path.ident();

    if let Ok(provider) = parse_internal::<IdentWithTypeArgs>(provider_type.to_token_stream())
        && provider.ident != "Self"
    {
        return Ok(Ident::new(
            &format!("__Impl_{}__", provider.ident),
            provider.ident.span(),
        ));
    }

    Ok(Ident::new(
        &format!("__Impl_{trait_ident}__"),
        trait_ident.span(),
    ))
}

fn take_helper_fns(items: &mut Vec<ImplItem>) -> syn::Result<Vec<ImplItemFn>> {
    let original = std::mem::take(items);
    let mut helpers = Vec::new();

    for item in original {
        match item {
            ImplItem::Fn(mut func) => {
                if take_helper_attr(&mut func.attrs)? {
                    helpers.push(func);
                } else {
                    items.push(ImplItem::Fn(func));
                }
            }
            other => {
                if has_helper_attr(&other) {
                    return Err(Error::new_spanned(
                        &other,
                        "#[helper] can only be applied to a method",
                    ));
                }
                items.push(other);
            }
        }
    }

    Ok(helpers)
}

fn take_helper_attr(attrs: &mut Vec<Attribute>) -> syn::Result<bool> {
    let mut found = false;

    let mut kept = Vec::new();
    for attr in attrs.drain(..) {
        if attr.path().is_ident("helper") {
            if !matches!(attr.meta, Meta::Path(_)) {
                return Err(Error::new_spanned(
                    attr,
                    "#[helper] does not take arguments",
                ));
            }
            found = true;
        } else {
            kept.push(attr);
        }
    }

    *attrs = kept;
    Ok(found)
}

fn has_helper_attr(item: &ImplItem) -> bool {
    let attrs = match item {
        ImplItem::Const(item) => &item.attrs,
        ImplItem::Fn(item) => &item.attrs,
        ImplItem::Type(item) => &item.attrs,
        ImplItem::Macro(item) => &item.attrs,
        _ => return false,
    };

    attrs.iter().any(|attr| attr.path().is_ident("helper"))
}
