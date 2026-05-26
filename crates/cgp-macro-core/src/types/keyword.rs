use core::fmt::Debug;
use core::marker::PhantomData;

use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Ident};

use crate::traits::IsKeyword;

pub struct Keyword<K: IsKeyword> {
    pub span: Span,
    pub phantom: PhantomData<K>,
}

impl<K: IsKeyword> Default for Keyword<K> {
    fn default() -> Self {
        Self {
            span: Span::call_site(),
            phantom: PhantomData,
        }
    }
}

impl<K: IsKeyword> Debug for Keyword<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("keyword").field("ident", &K::IDENT).finish()
    }
}

impl<K: IsKeyword> Clone for Keyword<K> {
    fn clone(&self) -> Self {
        Self {
            span: self.span,
            phantom: PhantomData,
        }
    }
}

impl<K> Parse for Keyword<K>
where
    K: IsKeyword,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        if ident != K::IDENT {
            return Err(Error::new_spanned(
                ident,
                format!("expect keyword: `{}`", K::IDENT),
            ));
        }

        Ok(Self {
            span: ident.span(),
            phantom: PhantomData,
        })
    }
}
