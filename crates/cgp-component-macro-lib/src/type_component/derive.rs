use alloc::format;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Plus};
use syn::{Ident, TypeParamBound};

pub fn derive_type_component(stream: TokenStream) -> syn::Result<TokenStream> {
    let spec: TypeComponentSpecs = syn::parse2(stream)?;

    Ok(do_derive_type_component(spec.ident, spec.bounds))
}

pub struct TypeComponentSpecs {
    pub ident: Ident,
    pub bounds: Punctuated<TypeParamBound, Plus>,
}

impl Parse for TypeComponentSpecs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;

        if input.is_empty() {
            return Ok(Self {
                ident,
                bounds: Punctuated::new(),
            });
        }

        let _: Colon = input.parse()?;

        let bounds = input.parse_terminated(TypeParamBound::parse, Plus)?;

        Ok(Self { ident, bounds })
    }
}

pub fn do_derive_type_component(
    ident: Ident,
    bounds: Punctuated<TypeParamBound, Plus>,
) -> TokenStream {
    let consumer_trait_name = Ident::new(&format!("Has{ident}Type"), ident.span());

    let provider_trait_name = Ident::new(&format!("Provide{ident}Type"), ident.span());

    let component_name = Ident::new(&format!("{ident}TypeComponent"), ident.span());

    quote! {
        pub struct #component_name;

        pub trait #consumer_trait_name {
            type #ident : #bounds ;
        }

        pub trait #provider_trait_name <Context> {
            type #ident : #bounds;
        }

        impl<Context, Components> #consumer_trait_name for Context
        where
            Context: DelegateComponent< #component_name, Components = Components >,
            Components: #provider_trait_name <Context>,
            Components:: #ident : #bounds,
        {
            type #ident = Components:: #ident;
        }

        impl<Context, Component, Delegate>
            #provider_trait_name <Context> for Component
        where
            Component: DelegateComponent< #component_name, Delegate = Delegate >,
            Components: #provider_trait_name <Context>,
            Delegate:: #ident : #bounds,
        {
            type #ident = Delegate:: #ident;
        }
    }
}
