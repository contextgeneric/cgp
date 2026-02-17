use quote::quote;
use syn::{Generics, Ident, ItemFn, ItemTrait, TraitItemFn, parse2};

use crate::cgp_fn::FunctionAttributes;

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

    let (_, type_generics, _) = generics.split_for_impl();

    let mut item_trait: ItemTrait = parse2(quote! {
        pub trait #trait_ident #type_generics {
            #trait_item_fn
        }
    })?;

    for extend in &attributes.extend {
        item_trait.supertraits.push(extend.clone());
    }

    Ok(item_trait)
}
