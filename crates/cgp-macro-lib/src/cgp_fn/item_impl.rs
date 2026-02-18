use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{Generics, Ident, ItemFn, ItemImpl, TypeParamBound, parse2};

use crate::cgp_fn::{
    FunctionAttributes, ImplicitArgField, build_implicit_args_bounds, derive_use_type_trait_bounds,
    substitute_abstract_type,
};

pub fn derive_item_impl(
    trait_ident: &Ident,
    item_fn: &ItemFn,
    implicit_args: &[ImplicitArgField],
    generics: &Generics,
    attributes: &FunctionAttributes,
) -> syn::Result<ItemImpl> {
    let type_generics = generics.split_for_impl().1;

    let mut item_impl: ItemImpl = parse2(quote! {
        impl #trait_ident #type_generics for __Context__ {
            #item_fn
        }
    })?;

    item_impl.generics = generics.clone();
    item_impl
        .generics
        .params
        .insert(0, parse2(quote! { __Context__ })?);

    {
        let mut bounds: Punctuated<TypeParamBound, Plus> = Punctuated::default();
        bounds.extend(attributes.extend.clone());

        for import in attributes.uses.iter() {
            bounds.push(parse2(quote! { #import })?);
        }

        if !bounds.is_empty() {
            item_impl
                .generics
                .make_where_clause()
                .predicates
                .push(parse2(quote! {
                    Self: #bounds
                })?);
        }
    }

    {
        let where_clause = item_impl.generics.make_where_clause();
        let bounds = build_implicit_args_bounds(implicit_args)?;

        where_clause.predicates.push(parse2(quote! {
            Self: #bounds
        })?);
    }

    if !attributes.use_type.is_empty() {
        item_impl = parse2(substitute_abstract_type(
            &quote! { Self },
            &attributes.use_type,
            item_impl.to_token_stream(),
        ))?;

        let bounds = derive_use_type_trait_bounds(&quote! { Self }, &attributes.use_type)?;
        let bounds = Punctuated::<TypeParamBound, Plus>::from_iter(bounds);

        item_impl
            .generics
            .make_where_clause()
            .predicates
            .push(parse2(quote! {
                Self: #bounds
            })?);
    }

    Ok(item_impl)
}
