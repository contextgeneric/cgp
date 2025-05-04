use core::iter;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Colon, Comma, Lt, Struct};
use syn::{braced, bracketed, Generics, Ident, Token, Type};

use crate::parse::ImplGenerics;

pub struct DelegateComponents {
    pub new_struct: Option<Struct>,
    pub target_type: Type,
    pub target_generics: ImplGenerics,
    pub entries: Punctuated<DelegateEntry<Type>, Comma>,
}

#[derive(Clone)]
pub struct DelegateEntry<T> {
    pub keys: Punctuated<DelegateKey<T>, Comma>,
    pub value: Type,
}

#[derive(Clone)]
pub struct DelegateKey<T> {
    pub ty: T,
    pub generics: ImplGenerics,
}

pub struct DelegateNewValue {
    pub wrapper_ident: Ident,
    pub struct_ident: Ident,
    pub struct_generics: Generics,
    pub entries: Punctuated<DelegateEntry<Type>, Comma>,
}

impl Parse for DelegateComponents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let new_struct = if input.peek(Struct) {
            Some(input.parse()?)
        } else {
            None
        };

        let target_type: Type = input.parse()?;

        let delegate_entries = {
            let content;
            braced!(content in input);
            Punctuated::parse_terminated(&content)?
        };

        Ok(Self {
            new_struct,
            target_type,
            target_generics,
            entries: delegate_entries,
        })
    }
}

impl<Type> Parse for DelegateEntry<Type>
where
    Type: Parse,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = if input.peek(Bracket) {
            let components_body;
            bracketed!(components_body in input);
            components_body.parse_terminated(DelegateKey::parse, Token![,])?
        } else {
            let component: DelegateKey<Type> = input.parse()?;
            Punctuated::from_iter(iter::once(component))
        };

        let _: Colon = input.parse()?;

        let source = input.parse()?;

        Ok(Self {
            keys: components,
            value: source,
        })
    }
}

impl<Type> Parse for DelegateKey<Type>
where
    Type: Parse,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let component_type: Type = input.parse()?;

        Ok(Self {
            ty: component_type,
            generics: component_generics,
        })
    }
}

impl Parse for DelegateNewValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let wrapper_ident = input.parse()?;

        let _: Lt = input.parse()?;

        let _: Struct = input.parse()?;

        let struct_ident = input.parse()?;

        let struct_generics = input.parse()?;

        let entries = {
            let content;
            braced!(content in input);

            Punctuated::parse_terminated(&content)?
        };

        Ok(Self {
            wrapper_ident,
            struct_ident,
            struct_generics,
            entries,
        })
    }
}

impl<Type> ToTokens for DelegateEntry<Type>
where
    Type: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let components = &self.keys;
        let source = &self.value;

        let count = components.len();

        #[allow(clippy::comparison_chain)]
        if count == 1 {
            tokens.append_all(quote! {
                #components : #source
            });
        } else if count > 1 {
            tokens.append_all(quote! {
                [
                    #components
                ] : #source
            });
        }
    }
}

impl<Type> ToTokens for DelegateKey<Type>
where
    Type: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generics.to_token_stream());
        tokens.extend(self.ty.to_token_stream());
    }
}

impl ToTokens for DelegateNewValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            wrapper_ident,
            struct_ident,
            struct_generics,
            entries,
        } = self;

        tokens.extend(quote! {
            #wrapper_ident <
                struct #struct_ident #struct_generics {
                    #entries
                }
            >
        });
    }
}
