use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{Generics, Ident, ItemFn, ItemImpl, TypeParamBound, parse2};

use crate::cgp_fn::{FunctionAttributes, ImplicitArgField};
use crate::derive_getter::derive_getter_constraint;
use crate::symbol::symbol_from_string;

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

    let where_clause = item_impl.generics.make_where_clause();

    let mut bounds: Punctuated<TypeParamBound, Plus> = Punctuated::default();
    bounds.extend(attributes.extend.clone());

    for import in attributes.uses.iter() {
        bounds.push(parse2(quote! { #import })?);
    }

    if !bounds.is_empty() {
        where_clause.predicates.push(parse2(quote! {
            Self: #bounds
        })?);
    }

    for arg in implicit_args {
        let field_symbol = symbol_from_string(&arg.field_name.to_string());

        let constraint = derive_getter_constraint(
            &arg.field_type,
            &arg.field_mut,
            &arg.field_mode,
            field_symbol.to_token_stream(),
            &None,
        )?;

        where_clause.predicates.push(parse2(quote! {
            Self: #constraint
        })?);
    }

    Ok(item_impl)
}
