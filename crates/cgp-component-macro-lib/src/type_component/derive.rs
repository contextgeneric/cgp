use alloc::format;
use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Colon, Plus, Pound};
use syn::{
    parse2, parse_quote, Attribute, Error, Generics, Ident, ItemImpl, ItemTrait, ItemType,
    TraitItem, TraitItemType, TypeParamBound,
};

use crate::derive_component::component_spec::{
    parse_component_from_entries, validate_component_entries, ComponentSpec,
};
use crate::derive_component::derive::{derive_component_with_ast, DerivedComponent};
use crate::derive_component::entry::Entries;
use crate::derive_provider::derive_is_provider_for;

pub fn derive_type_component(attrs: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let Entries { mut entries } = syn::parse2(attrs)?;

    let consumer_trait: ItemTrait = syn::parse2(body)?;

    let item_type = extract_item_type(&consumer_trait)?.clone();

    entries.entry("provider".into()).or_insert_with(|| {
        let provider_name = Ident::new(
            &format!("{}TypeProvider", item_type.ident),
            item_type.ident.span(),
        );
        parse_quote!( #provider_name )
    });

    let spec = parse_component_from_entries(&entries)?;

    let component = derive_component_with_ast(&spec, consumer_trait)?;

    let alias_type = derive_type_alias(&component.consumer_trait, &spec.context_type, &item_type)?;

    let use_type_impl = derive_use_type_impl(&component.provider_trait, &item_type)?;

    Ok(quote! {
        #component

        #alias_type

        #use_type_impl
    })
}

pub fn extract_item_type(consumer_trait: &ItemTrait) -> syn::Result<&TraitItemType> {
    if consumer_trait.items.len() != 1 {
        return Err(Error::new(
            consumer_trait.span(),
            "type trait should contain exactly one associated type item",
        ));
    }

    match consumer_trait.items.get(0) {
        Some(TraitItem::Type(item_type)) => {
            if !item_type.generics.params.is_empty() || item_type.generics.where_clause.is_some() {
                return Err(Error::new(
                    consumer_trait.span(),
                    "generic associated type and where clause are not supported",
                ));
            }

            Ok(item_type)
        }
        _ => Err(Error::new(
            consumer_trait.span(),
            "type trait should contain exactly one associated type item",
        )),
    }
}

pub fn derive_type_alias(
    consumer_trait: &ItemTrait,
    context_name: &Ident,
    item_type: &TraitItemType,
) -> syn::Result<ItemType> {
    let consumer_trait_name = &consumer_trait.ident;

    let (_, type_generics, _) = consumer_trait.generics.split_for_impl();

    let type_generics: Generics = parse2(type_generics.to_token_stream())?;

    let type_generics_params = &type_generics.params;

    let type_name = &item_type.ident;
    let alias_name = Ident::new(&format!("{}Of", type_name), type_name.span());

    let alias_type: ItemType = parse2(quote! {
        pub type #alias_name < #context_name, #type_generics_params > =
            < #context_name as #consumer_trait_name #type_generics > :: #type_name ;
    })?;

    Ok(alias_type)
}

pub fn derive_use_type_impl(
    provider_trait: &ItemTrait,
    item_type: &TraitItemType,
) -> syn::Result<ItemImpl> {
    let provider_trait_name = &provider_trait.ident;

    let (impl_generics, type_generics, where_clause) = provider_trait.generics.split_for_impl();

    let impl_generics_params = parse2::<Generics>(impl_generics.to_token_stream())?.params;

    let predicates = where_clause
        .map(|c| c.predicates.clone())
        .unwrap_or_default();

    let type_name = &item_type.ident;

    let type_bounds = if item_type.bounds.is_empty() {
        TokenStream::new()
    } else {
        let bounds = &item_type.bounds;

        quote! {
            #type_name: #bounds,
        }
    };

    let use_type_impl: ItemImpl = parse2(quote! {
        impl< #type_name, #impl_generics_params >
            #provider_trait_name #type_generics
            for UseType< #type_name >
        where
            #type_bounds
            #predicates
        {
            type #type_name = #type_name;
        }
    })?;

    Ok(use_type_impl)
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
