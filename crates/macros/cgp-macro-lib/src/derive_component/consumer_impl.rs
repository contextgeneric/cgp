use alloc::boxed::Box;
use alloc::vec::Vec;

use cgp_macro_core::functions::{signature_to_delegated_impl_item_fn, trait_to_impl_item_type};
use cgp_macro_core::types::generics::TypeGenerics;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::token::{Brace, Eq, For, Impl};
use syn::{
    Error, Ident, ImplItem, ImplItemConst, ItemImpl, ItemTrait, Path, TraitItem, Type, Visibility,
    parse2,
};

pub fn derive_consumer_impl(
    consumer_trait: &ItemTrait,
    provider_name: &Ident,
    context_type: &Ident,
) -> syn::Result<ItemImpl> {
    let consumer_name = &consumer_trait.ident;

    let consumer_type_generics = TypeGenerics::try_from(&consumer_trait.generics)?;

    let provider_trait_path: Type = {
        let mut provider_type_generics = consumer_type_generics.clone();
        provider_type_generics
            .generics
            .params
            .insert(0, parse2(quote!(#context_type))?);

        parse2(quote!(#provider_name #provider_type_generics))?
    };

    let generics_for_impl = {
        let mut generics = consumer_trait.generics.clone();

        generics.params.insert(0, parse2(quote!(#context_type))?);

        let where_clause = generics.make_where_clause();

        if !consumer_trait.supertraits.is_empty() {
            let supertrait_constraints = consumer_trait.supertraits.clone();
            where_clause.predicates.push(parse2(quote! {
                #context_type : #supertrait_constraints
            })?);
        }

        where_clause.predicates.push(parse2(quote! {
            #context_type : #provider_trait_path
        })?);

        generics
    };

    let mut impl_items: Vec<ImplItem> = Vec::new();

    for trait_item in consumer_trait.items.iter() {
        match trait_item {
            TraitItem::Fn(trait_fn) => {
                let impl_fn = signature_to_delegated_impl_item_fn(
                    &trait_fn.sig,
                    &parse2(quote!(#context_type))?,
                )?;

                impl_items.push(ImplItem::Fn(impl_fn));
            }
            TraitItem::Type(trait_type) => {
                let type_name = &trait_type.ident;
                let type_generics = trait_type.generics.split_for_impl().1;
                let delegate_type = parse2(quote!(
                    < #context_type as #provider_trait_path > :: #type_name #type_generics
                ))?;

                let impl_type = trait_to_impl_item_type(trait_type, delegate_type);

                impl_items.push(ImplItem::Type(impl_type));
            }
            TraitItem::Const(trait_item_const) => {
                let const_ident = &trait_item_const.ident;
                let (_, type_generics, _) = trait_item_const.generics.split_for_impl();

                let impl_expr = parse2(quote! {
                    < #context_type as #provider_trait_path > :: #const_ident #type_generics
                })?;

                let impl_item_const = ImplItemConst {
                    attrs: trait_item_const.attrs.clone(),
                    vis: Visibility::Inherited,
                    defaultness: None,
                    const_token: trait_item_const.const_token,
                    ident: trait_item_const.ident.clone(),
                    generics: trait_item_const.generics.clone(),
                    colon_token: trait_item_const.colon_token,
                    ty: trait_item_const.ty.clone(),
                    eq_token: Eq(Span::call_site()),
                    expr: impl_expr,
                    semi_token: trait_item_const.semi_token,
                };

                impl_items.push(ImplItem::Const(impl_item_const));
            }
            _ => {
                return Err(Error::new(
                    trait_item.span(),
                    format!("unsupported trait item: {trait_item:?}"),
                ));
            }
        }
    }

    let consumer_trait_path: Path = parse2(quote!( #consumer_name #consumer_type_generics ))?;

    let item_impl = ItemImpl {
        attrs: consumer_trait.attrs.clone(),
        defaultness: None,
        unsafety: consumer_trait.unsafety,
        impl_token: Impl::default(),
        generics: generics_for_impl,
        trait_: Some((None, consumer_trait_path, For::default())),
        self_ty: Box::new(parse2(quote!(#context_type))?),
        brace_token: Brace::default(),
        items: impl_items,
    };

    Ok(item_impl)
}
