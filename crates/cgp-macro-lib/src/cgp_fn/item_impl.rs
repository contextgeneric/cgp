use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{Generics, Ident, ItemFn, ItemImpl, TypeParamBound, parse2};

use crate::cgp_fn::{
    FunctionAttributes, ImplicitArgField, apply_use_type_attributes_to_item_impl,
    build_implicit_args_bounds,
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

    if !attributes.extend_where.is_empty() {
        item_impl
            .generics
            .make_where_clause()
            .predicates
            .extend(attributes.extend_where.clone());
    }

    if !implicit_args.is_empty() {
        let where_clause = item_impl.generics.make_where_clause();
        let bounds = build_implicit_args_bounds(implicit_args)?;

        where_clause.predicates.push(parse2(quote! {
            Self: #bounds
        })?);
    }

    if !attributes.use_type.is_empty() {
        item_impl = apply_use_type_attributes_to_item_impl(&item_impl, &attributes.use_type)?;
    }

    Ok(item_impl)
}
