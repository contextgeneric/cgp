use alloc::boxed::Box;
use alloc::vec::Vec;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Brace, Comma, Eq, For, Impl, Plus};
use syn::{
    Error, GenericParam, Ident, ImplItem, ImplItemConst, ItemImpl, ItemTrait, Path, TraitItem,
    TypeParamBound, Visibility, parse2,
};

use crate::derive_component::delegate_fn::derive_delegated_fn_impl;
use crate::derive_component::delegate_type::derive_delegate_type_impl;
use crate::parse::parse_is_provider_params;

pub fn derive_provider_impl(
    context_type: &Ident,
    consumer_trait: &ItemTrait,
    provider_trait: &ItemTrait,
    component_name: &Ident,
    component_params: &Punctuated<Ident, Comma>,
) -> syn::Result<ItemImpl> {
    let provider_name = &provider_trait.ident;

    let provider_type = Ident::new("__Provider__", Span::call_site());

    let delegate_constraint = quote! {
        DelegateComponent< #component_name < #component_params > >
    };

    let delegate_type = quote! {
        < #provider_type as #delegate_constraint > :: Delegate
    };

    let provider_type_generics = provider_trait.generics.split_for_impl().1;

    let impl_generics = {
        let mut impl_generics = provider_trait.generics.clone();

        impl_generics
            .params
            .insert(0, parse2(quote!(#provider_type))?);

        {
            let is_provider_params = parse_is_provider_params(&consumer_trait.generics)?;

            let mut delegate_constraints: Punctuated<TypeParamBound, Plus> = Punctuated::default();

            delegate_constraints.push(parse2(delegate_constraint)?);

            delegate_constraints.push(parse2(quote!(
                IsProviderFor< #component_name < #component_params >, #context_type, ( #is_provider_params ) >
            ))?);

            let provider_constraint: TypeParamBound = parse2(quote! {
                #provider_name #provider_type_generics
            })?;

            let where_clause = impl_generics.make_where_clause();

            where_clause.predicates.push(parse2(quote! {
                #provider_type : #delegate_constraints
            })?);

            where_clause.predicates.push(parse2(quote! {
                #delegate_type : #provider_constraint
            })?);
        }

        impl_generics
    };

    let impl_items = derive_provider_item_impls(provider_trait, &delegate_type)?;

    let trait_path: Path = parse2(quote!( #provider_name #provider_type_generics ))?;

    let item = ItemImpl {
        attrs: provider_trait.attrs.clone(),
        defaultness: None,
        unsafety: provider_trait.unsafety,
        impl_token: Impl::default(),
        generics: impl_generics,
        trait_: Some((None, trait_path, For::default())),
        self_ty: Box::new(parse2(quote!(#provider_type))?),
        brace_token: Brace::default(),
        items: impl_items,
    };

    Ok(item)
}

pub fn derive_provider_item_impls(
    provider_trait: &ItemTrait,
    delegate_type: &TokenStream,
) -> syn::Result<Vec<ImplItem>> {
    let provider_name = &provider_trait.ident;
    let provider_type_generics = provider_trait.generics.split_for_impl().1;

    let mut impl_items: Vec<ImplItem> = Vec::new();

    for trait_item in provider_trait.items.iter() {
        match &trait_item {
            TraitItem::Fn(trait_fn) => {
                let impl_fn = derive_delegated_fn_impl(&trait_fn.sig, delegate_type)?;

                impl_items.push(ImplItem::Fn(impl_fn))
            }
            TraitItem::Type(trait_type) => {
                let type_name = &trait_type.ident;

                let type_generics = {
                    let mut type_generics = trait_type.generics.clone();
                    type_generics.where_clause = None;

                    for param in &mut type_generics.params {
                        if let GenericParam::Type(type_param) = param {
                            type_param.bounds.clear();
                        }
                    }

                    type_generics
                };

                let impl_type = derive_delegate_type_impl(
                    trait_type,
                    parse2(quote!(
                        < #delegate_type as #provider_name #provider_type_generics > :: #type_name #type_generics
                    ))?,
                );

                impl_items.push(ImplItem::Type(impl_type));
            }
            TraitItem::Const(trait_item_const) => {
                let const_ident = &trait_item_const.ident;
                let (_, type_generics, _) = trait_item_const.generics.split_for_impl();

                let impl_expr = parse2(quote! {
                    < #delegate_type as #provider_name #provider_type_generics > :: #const_ident #type_generics
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

    Ok(impl_items)
}
