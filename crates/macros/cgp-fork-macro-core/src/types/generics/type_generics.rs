use core::ops::{Deref, DerefMut};

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Generics};

use crate::functions::parse_internal;

/// A validated newtype around [`syn::Generics`] restricted to a definition-site
/// generic list (no bounds). Because it `Deref`s to [`syn::Generics`], the full
/// `syn` API (`split_for_impl`, mutating `params`, …) is available, and its
/// `TryFrom<&Generics>` adapts a generic list already parsed off an item.
///
/// Prefer this when adapting or manipulating an existing `syn::Generics`. When
/// instead *parsing tokens* and you want strict, kind-classified parameters,
/// prefer [`TypeGenericParams`]. The two are intentionally kept separate; see
/// [`TypeGenericParams`] for the full rationale (notably, `TryFrom` here
/// normalizes a `const N: T` parameter down to a bare `N`).
///
/// [`TypeGenericParams`]: crate::types::ident::TypeGenericParams
#[derive(Debug, Clone, Default)]
pub struct TypeGenerics {
    pub generics: Generics,
}

impl Deref for TypeGenerics {
    type Target = Generics;

    fn deref(&self) -> &Generics {
        &self.generics
    }
}

impl DerefMut for TypeGenerics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.generics
    }
}

impl Parse for TypeGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generics: Generics = input.parse()?;

        let (_, type_generics, _) = generics.split_for_impl();

        let generics2: Generics = parse_internal(type_generics.to_token_stream())?;

        if generics != generics2 {
            return Err(Error::new_spanned(generics, "invalid type generics syntax"));
        }

        Ok(Self { generics })
    }
}

impl<'a> TryFrom<&'a Generics> for TypeGenerics {
    type Error = syn::Error;

    fn try_from(generics: &'a Generics) -> syn::Result<Self> {
        let (_, type_generics, _) = generics.split_for_impl();
        let generics = parse_internal(type_generics.to_token_stream())?;
        Ok(Self { generics })
    }
}

impl ToTokens for TypeGenerics {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.generics.to_tokens(tokens);
    }
}
