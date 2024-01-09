use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, For, Impl, Plus};
use syn::{
    parse_quote, GenericParam, Ident, ImplItem, ItemImpl, ItemTrait, Path, TraitItem,
    TypeParamBound,
};

use crate::helper::delegate_fn::derive_delegated_fn_impl;
use crate::helper::delegate_type::derive_delegate_type_impl;

pub fn derive_provider_impl(
    provider_trait: &ItemTrait,
    component_name: &Ident,
    component_params: &Punctuated<Ident, Comma>,
) -> ItemImpl {
    let provider_name = &provider_trait.ident;

    let component_type = Ident::new("Component", Span::call_site());

    let provider_generic_args = {
        let mut generic_args: Punctuated<Ident, Comma> = Punctuated::new();

        for param in provider_trait.generics.params.iter() {
            match param {
                GenericParam::Type(ty) => {
                    generic_args.push(ty.ident.clone());
                }
                GenericParam::Const(arg) => {
                    generic_args.push(arg.ident.clone());
                }
                GenericParam::Lifetime(_life) => {
                    unimplemented!()
                }
            }
        }

        generic_args
    };


    let impl_generics = {
        let mut impl_generics = provider_trait.generics.clone();

        impl_generics
            .params
            .insert(0, parse_quote!(#component_type));

        {
            let delegate_constraint: Punctuated<TypeParamBound, Plus> = parse_quote! {
                DelegateComponent< #component_name < #component_params > >
            };

            let provider_constraint: Punctuated<TypeParamBound, Plus> = parse_quote! {
                #provider_name < #provider_generic_args >
            };

            if let Some(where_clause) = &mut impl_generics.where_clause {
                where_clause.predicates.push(parse_quote! {
                    #component_type : #delegate_constraint
                });

                where_clause.predicates.push(parse_quote! {
                    #component_type :: Delegate : #provider_constraint
                });
            } else {
                impl_generics.where_clause = Some(parse_quote! {
                    where
                        #component_type : #delegate_constraint,
                        #component_type :: Delegate : #provider_constraint
                });
            }
        }

        impl_generics
    };

    let mut impl_items: Vec<ImplItem> = Vec::new();

    for trait_item in provider_trait.items.iter() {
        match trait_item {
            TraitItem::Fn(trait_fn) => {
                let impl_fn = derive_delegated_fn_impl(
                    &trait_fn.sig,
                    &parse_quote!(#component_type :: Delegate),
                );

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
                    &trait_type,
                    parse_quote!(
                        < #component_type :: Delegate as #provider_name < #provider_generic_args > > :: #type_name #type_generics
                    ),
                );

                impl_items.push(ImplItem::Type(impl_type));
            }
            _ => {}
        }
    }

    let trait_path: Path = parse_quote!( #provider_name < #provider_generic_args > );

    ItemImpl {
        attrs: provider_trait.attrs.clone(),
        defaultness: None,
        unsafety: provider_trait.unsafety,
        impl_token: Impl::default(),
        generics: impl_generics,
        trait_: Some((None, trait_path, For::default())),
        self_ty: Box::new(parse_quote!(#component_type)),
        brace_token: Brace::default(),
        items: impl_items,
    }
}
