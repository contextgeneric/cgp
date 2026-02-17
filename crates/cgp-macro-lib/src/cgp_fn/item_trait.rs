use quote::{ToTokens, quote};
use syn::{Generics, Ident, ItemFn, ItemTrait, TraitItemFn, parse2};

use crate::cgp_fn::{FunctionAttributes, substitute_abstract_type};

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

    if !attributes.use_type.is_empty() {
        let mut type_idents = Vec::new();

        for use_type in attributes.use_type.iter() {
            item_trait
                .supertraits
                .push(parse2(use_type.trait_path.to_token_stream())?);

            type_idents.extend(use_type.type_idents.clone());
        }

        item_trait = parse2(substitute_abstract_type(
            &quote! { Self },
            &type_idents,
            item_trait.to_token_stream(),
        ))?;
    }

    Ok(item_trait)
}
