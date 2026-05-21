use syn::Ident;
use syn::parse::Parse;
use syn::token::Semi;

use crate::define_keyword;

define_keyword!(Namespace, NamespaceKeyword, "namespace");

pub struct NamespaceDelegateEntry {
    pub namespace: Namespace,
    pub ident: Ident,
    pub semi: Semi,
}

impl Parse for NamespaceDelegateEntry {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let namespace = input.parse()?;
        let ident = input.parse()?;
        let semi = input.parse()?;

        Ok(Self {
            namespace,
            ident,
            semi,
        })
    }
}
