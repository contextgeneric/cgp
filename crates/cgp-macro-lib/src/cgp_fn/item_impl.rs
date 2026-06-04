use cgp_macro_core::traits::AddTypeParamBounds;
use cgp_macro_core::types::attributes::FunctionAttributes;
use cgp_macro_core::types::implicits::ImplicitArgFields;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{Generics, Ident, ItemFn, ItemImpl, Type, TypeParamBound, parse_quote, parse2};

pub fn derive_item_impl(
    trait_ident: &Ident,
    item_fn: &ItemFn,
    implicit_args: &ImplicitArgFields,
    generics: &Generics,
    attributes: &FunctionAttributes,
) -> syn::Result<ItemImpl> {
    let type_generics = generics.split_for_impl().1;

    let self_type: Type = parse_quote!(Self);

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

    item_impl
        .generics
        .params
        .extend(attributes.impl_generics.clone());

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

    implicit_args.add_type_param_bounds(&self_type, &mut item_impl.generics)?;

    attributes.use_type.transform_item_impl(&mut item_impl)?;
    attributes
        .use_provider
        .add_type_param_bounds(&self_type, &mut item_impl.generics)?;

    Ok(item_impl)
}
