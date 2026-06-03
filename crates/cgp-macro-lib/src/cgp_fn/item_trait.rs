use cgp_macro_core::types::attributes::UseTypeAttributes;
use quote::{ToTokens, quote};
use syn::{Generics, Ident, ItemFn, ItemTrait, TraitItemFn, parse_quote, parse2};

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

    if !attributes.use_type.attributes.is_empty() {
        expand_use_type_attributes_on_trait(&mut item_trait, &attributes.use_type)?;
    }

    Ok(item_trait)
}

pub fn expand_use_type_attributes_on_trait(
    item_trait: &mut ItemTrait,
    use_type_specs: &UseTypeAttributes,
) -> syn::Result<()> {
    use_type_specs.substitute_abstract_types_in_item_trait(item_trait);

    for use_type in use_type_specs.attributes.iter() {
        if use_type.context_type != parse_quote! { Self } {
            continue;
        }

        item_trait
            .supertraits
            .push(parse2(use_type.trait_path.to_token_stream())?);
    }

    Ok(())
}
