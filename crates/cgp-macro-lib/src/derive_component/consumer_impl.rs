use alloc::boxed::Box;
use alloc::vec::Vec;

use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::token::{Brace, Eq, For, Impl};
use syn::{
    Error, GenericParam, Generics, Ident, ImplItem, ImplItemConst, ItemImpl, ItemTrait, Path,
    TraitItem, TypeParamBound, Visibility, parse2,
};

use crate::derive_component::delegate_fn::derive_delegated_fn_impl;
use crate::derive_component::delegate_type::derive_delegate_type_impl;

pub fn derive_consumer_impl(
    consumer_trait: &ItemTrait,
    provider_name: &Ident,
    context_type: &Ident,
) -> syn::Result<ItemImpl> {
    let consumer_name = &consumer_trait.ident;

    let consumer_type_generics = {
        let (_, type_generics, _) = consumer_trait.generics.split_for_impl();
        let generics: Generics = parse2(type_generics.to_token_stream())?;

        generics.params
    };

    let provider_type_generics = {
        let mut generic_args = consumer_type_generics.clone();

        generic_args.insert(0, parse2(quote!(#context_type))?);

        generic_args
    };

    let generics_for_impl = {
        let mut generics = consumer_trait.generics.clone();

        generics.params.insert(0, parse2(quote!(#context_type))?);

        {
            let supertrait_constraints = consumer_trait.supertraits.clone();

            if !supertrait_constraints.is_empty() {
                match &mut generics.where_clause {
                    Some(where_clause) => {
                        where_clause.predicates.push(parse2(quote! {
                            #context_type : #supertrait_constraints
                        })?);
                    }
                    _ => {
                        generics.where_clause = Some(parse2(quote! {
                            where #context_type : #supertrait_constraints
                        })?);
                    }
                }
            }
        }

        {
            let has_component_constraint: TypeParamBound = parse2(quote! {
                HasCgpProvider
            })?;

            let provider_constraint: TypeParamBound = parse2(quote! {
                #provider_name < #provider_type_generics >
            })?;

            match &mut generics.where_clause {
                Some(where_clause) => {
                    where_clause.predicates.push(parse2(quote! {
                        #context_type : #has_component_constraint
                    })?);

                    where_clause.predicates.push(parse2(quote! {
                        #context_type :: CgpProvider : #provider_constraint
                    })?);
                }
                _ => {
                    generics.where_clause = Some(parse2(quote! {
                        where
                            #context_type : #has_component_constraint,
                            #context_type :: CgpProvider : #provider_constraint
                    })?);
                }
            }
        }

        generics
    };

    let mut impl_items: Vec<ImplItem> = Vec::new();

    for trait_item in consumer_trait.items.iter() {
        match trait_item {
            TraitItem::Fn(trait_fn) => {
                let impl_fn = derive_delegated_fn_impl(
                    &trait_fn.sig,
                    &parse2(quote!(#context_type :: CgpProvider))?,
                )?;

                impl_items.push(ImplItem::Fn(impl_fn));
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
                        < #context_type :: CgpProvider as #provider_name < #provider_type_generics > > :: #type_name #type_generics
                    ))?,
                );

                impl_items.push(ImplItem::Type(impl_type));
            }
            TraitItem::Const(trait_item_const) => {
                let const_ident = &trait_item_const.ident;
                let (_, type_generics, _) = trait_item_const.generics.split_for_impl();

                let impl_expr = parse2(quote! {
                    < #context_type :: CgpProvider as #provider_name < #provider_type_generics > > :: #const_ident #type_generics
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

    let trait_path: Path = parse2(quote!( #consumer_name < #consumer_type_generics > ))?;

    let item_impl = ItemImpl {
        attrs: consumer_trait.attrs.clone(),
        defaultness: None,
        unsafety: consumer_trait.unsafety,
        impl_token: Impl::default(),
        generics: generics_for_impl,
        trait_: Some((None, trait_path, For::default())),
        self_ty: Box::new(parse2(quote!(#context_type))?),
        brace_token: Brace::default(),
        items: impl_items,
    };

    Ok(item_impl)
}
