use core::iter;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Colon, Comma, Lt};
use syn::{braced, bracketed, Token, Type};

use crate::parse::ImplGenerics;

pub struct DelegateComponents {
    pub target_type: Type,
    pub target_generics: ImplGenerics,
    pub delegate_entries: Punctuated<DelegateComponentEntry<Type>, Comma>,
}

#[derive(Clone)]
pub struct DelegateComponentEntry<T> {
    pub components: Punctuated<DelegateComponentName<T>, Comma>,
    pub source: Type,
}

#[derive(Clone)]
pub struct DelegateComponentName<T> {
    pub component_type: T,
    pub component_generics: ImplGenerics,
}

impl Parse for DelegateComponents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let target_type: Type = input.parse()?;

        let delegate_entries = {
            let content;
            braced!(content in input);
            Punctuated::parse_terminated(&content)?
        };

        Ok(Self {
            target_type,
            target_generics,
            delegate_entries,
        })
    }
}

impl<Type> Parse for DelegateComponentEntry<Type>
where
    Type: Parse,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = if input.peek(Bracket) {
            let components_body;
            bracketed!(components_body in input);
            components_body.parse_terminated(DelegateComponentName::parse, Token![,])?
        } else {
            let component: DelegateComponentName<Type> = input.parse()?;
            Punctuated::from_iter(iter::once(component))
        };

        let _: Colon = input.parse()?;

        let source = input.parse()?;

        Ok(Self { components, source })
    }
}

impl<Type> Parse for DelegateComponentName<Type>
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
            component_type,
            component_generics,
        })
    }
}

impl<Type> ToTokens for DelegateComponentEntry<Type>
where
    Type: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let components = &self.components;
        let source = &self.source;

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

impl<Type> ToTokens for DelegateComponentName<Type>
where
    Type: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.component_generics.to_token_stream());
        tokens.extend(self.component_type.to_token_stream());
    }
}
