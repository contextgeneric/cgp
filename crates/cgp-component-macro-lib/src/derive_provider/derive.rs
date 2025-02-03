use std::collections::BTreeMap;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Comma, For};
use syn::{
    parse_quote, AngleBracketedGenericArguments, Error, GenericArgument, ItemImpl, Path,
    PathArguments, Type,
};

use crate::derive_provider::replace_provider_in_generics;

pub fn derive_provider(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let component_name: Type = syn::parse2(attr)?;

    let provider_impl: ItemImpl = syn::parse2(body)?;

    let is_provider_for_impl: ItemImpl = derive_is_provider_for(&component_name, &provider_impl)?;

    let result = quote! {
        #provider_impl
        #is_provider_for_impl
    };

    Ok(result)
}

pub fn derive_is_provider_for(
    component_name: &Type,
    provider_impl: &ItemImpl,
) -> syn::Result<ItemImpl> {
    let provider_path = provider_impl
        .trait_
        .as_ref()
        .ok_or_else(|| {
            Error::new(
                provider_impl.span(),
                "provider impl should contain trait path",
            )
        })?
        .1
        .segments
        .last()
        .ok_or_else(|| {
            Error::new(
                provider_impl.span(),
                "provider impl should contain trait path containing generic parameters",
            )
        })?;

    let provider_map = BTreeMap::from([(provider_path.ident.clone(), component_name.clone())]);

    let is_provider_generics: AngleBracketedGenericArguments = match &provider_path.arguments {
        PathArguments::AngleBracketed(generics) => {
            let mut generic_args = generics.clone().args.into_iter();

            let context_arg = generic_args.next().ok_or_else(|| {
                Error::new(
                    provider_impl.span(),
                    "provider impl should contain trait path containing at least one generic parameter",
                )
            })?;

            let rest: Punctuated<GenericArgument, Comma> = generic_args.collect();

            parse_quote!( < #component_name, #context_arg, ( #rest ) > )
        }
        _ => {
            return Err(Error::new(
                provider_impl.span(),
                "provider impl should contain trait path containing generic parameters",
            ));
        }
    };

    let is_provider_path: Path = parse_quote!( IsProviderFor #is_provider_generics );

    let mut is_provider_impl = provider_impl.clone();

    is_provider_impl.attrs.clear();
    is_provider_impl.items.clear();
    is_provider_impl.defaultness = None;
    is_provider_impl.unsafety = None;

    is_provider_impl.trait_ = Some((None, is_provider_path, For(Span::call_site())));

    replace_provider_in_generics(&provider_map, &mut is_provider_impl.generics);

    Ok(is_provider_impl)
}
