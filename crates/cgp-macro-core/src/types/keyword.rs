use core::marker::PhantomData;

use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Ident};

use crate::traits::IsKeyword;

pub struct Keyword<K: IsKeyword> {
    pub span: Span,
    pub phantom: PhantomData<K>,
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
