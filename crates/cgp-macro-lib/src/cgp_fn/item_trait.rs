use quote::{ToTokens, quote};
use syn::{Generics, Ident, ItemFn, ItemTrait, TraitItemFn, parse2};

use crate::cgp_fn::{FunctionAttributes, UseTypeSpec, substitute_abstract_type};

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

    item_trait.supertraits.extend(attributes.extend.clone());

    if !attributes.use_type.is_empty() {
        item_trait = expand_use_type_attributes_on_trait(&item_trait, &attributes.use_type)?;
    }

    Ok(item_trait)
}

pub fn expand_use_type_attributes_on_trait(
    item_trait: &ItemTrait,
    use_type_specs: &[UseTypeSpec],
) -> syn::Result<ItemTrait> {
    let mut item_trait: ItemTrait = parse2(substitute_abstract_type(
        &quote! { Self },
        use_type_specs,
        item_trait.to_token_stream(),
    ))?;

    for use_type in use_type_specs.iter() {
        item_trait
            .supertraits
            .push(parse2(use_type.trait_path.to_token_stream())?);
    }

    Ok(item_trait)
}
