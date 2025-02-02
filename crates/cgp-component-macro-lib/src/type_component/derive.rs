use alloc::format;
use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Plus, Pound};
use syn::{parse_quote, Attribute, Ident, ItemImpl, ItemTrait, ItemType, TypeParamBound};

use crate::derive_provider::derive_is_provider_for;

pub fn derive_type_component(stream: TokenStream) -> syn::Result<TokenStream> {
    let spec: TypeComponentSpecs = syn::parse2(stream)?;

    do_derive_type_component(spec.attributes, spec.ident, spec.bounds)
}

pub struct TypeComponentSpecs {
    pub attributes: Vec<Attribute>,
    pub ident: Ident,
    pub bounds: Punctuated<TypeParamBound, Plus>,
}

impl Parse for TypeComponentSpecs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = {
            let lookahead = input.lookahead1();
            if lookahead.peek(Pound) {
                input.call(Attribute::parse_outer)?
            } else {
                Vec::new()
            }
        };

        let ident = input.parse()?;

        if input.is_empty() {
            return Ok(Self {
                attributes,
                ident,
                bounds: Punctuated::new(),
            });
        }

        let _: Colon = input.parse()?;

        let bounds = input.parse_terminated(TypeParamBound::parse, Plus)?;

        Ok(Self {
            attributes,
            ident,
            bounds,
        })
    }
}

pub fn do_derive_type_component(
    attributes: Vec<Attribute>,
    ident: Ident,
    bounds: Punctuated<TypeParamBound, Plus>,
) -> syn::Result<TokenStream> {
    let consumer_trait_name = Ident::new(&format!("Has{ident}Type"), ident.span());

    let provider_trait_name = Ident::new(&format!("Provide{ident}Type"), ident.span());

    let alias_name = Ident::new(&format!("{ident}Of"), ident.span());

    let component_name = Ident::new(&format!("{ident}TypeComponent"), ident.span());

    let alias_type: ItemType = parse_quote! {
        pub type #alias_name <__Context__> = <__Context__ as #consumer_trait_name>:: #ident;
    };

    let mut consumer_trait: ItemTrait = parse_quote! {
        pub trait #consumer_trait_name {
            type #ident : #bounds ;
        }
    };

    consumer_trait.attrs = attributes;

    let provider_trait: ItemTrait = parse_quote! {
        pub trait #provider_trait_name <__Context__> {
            type #ident : #bounds;
        }
    };

    let consumer_impl: ItemImpl = parse_quote! {
        impl<__Context__, __Components__> #consumer_trait_name for __Context__
        where
            __Context__: HasComponents< Components = __Components__ >,
            __Components__: #provider_trait_name <__Context__>,
            __Components__:: #ident : #bounds,
        {
            type #ident = __Components__:: #ident;
        }
    };

    let provider_impl: ItemImpl = parse_quote! {
        impl<__Context__, Component, Delegate>
            #provider_trait_name <__Context__> for Component
        where
            Component: DelegateComponent< #component_name, Delegate = Delegate >,
            Delegate: #provider_trait_name <__Context__>,
            Delegate:: #ident : #bounds,
        {
            type #ident = Delegate:: #ident;
        }
    };

    let with_provider_impl: ItemImpl = parse_quote! {
        impl<__Context__, Provider, #ident> #provider_trait_name <__Context__>
            for WithProvider<Provider>
        where
            Provider: ProvideType<__Context__, #component_name, Type = #ident >,
            #ident: #bounds,
        {
            type #ident = #ident;
        }
    };

    let is_provider_for_with_provider_impl =
        derive_is_provider_for(&parse_quote!(#component_name), &with_provider_impl)?;

    let use_type_impl: ItemImpl = parse_quote! {
        impl<__Context__, #ident> #provider_trait_name <__Context__>
            for UseType<#ident>
        where
            #ident: #bounds,
        {
            type #ident = #ident;
        }
    };

    let is_provider_for_use_type_impl =
        derive_is_provider_for(&parse_quote!(#component_name), &use_type_impl)?;

    Ok(quote! {
        pub struct #component_name;

        #consumer_trait

        #alias_type

        #provider_trait

        #consumer_impl

        #provider_impl

        #with_provider_impl

        #is_provider_for_with_provider_impl

        #use_type_impl

        #is_provider_for_use_type_impl
    })
}
