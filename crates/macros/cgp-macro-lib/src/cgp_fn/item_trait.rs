use cgp_macro_core::types::attributes::FunctionAttributes;
use quote::quote;
use syn::{Generics, Ident, ItemFn, ItemTrait, TraitItemFn, parse2};

pub fn derive_item_trait(
    trait_ident: &Ident,
    item_fn: &ItemFn,
    generics: &Generics,
    attributes: &FunctionAttributes,
) -> syn::Result<ItemTrait> {
    let trait_item_fn = TraitItemFn {
        attrs: item_fn.attrs.clone(),
        sig: item_fn.sig.clone(),
        default: None,
        semi_token: None,
    };

    let mut item_trait: ItemTrait = parse2(quote! {
        pub trait #trait_ident {
            #trait_item_fn
        }
    })?;

    item_trait.generics = generics.clone();
    item_trait.generics.where_clause = None;

    item_trait.supertraits.extend(attributes.extend.clone());

    if !attributes.extend_where.is_empty() {
        item_trait
            .generics
            .make_where_clause()
            .predicates
            .extend(attributes.extend_where.clone());
    }

    attributes.use_type.transform_item_trait(&mut item_trait)?;

    Ok(item_trait)
}
