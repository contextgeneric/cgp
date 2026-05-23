use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Ident, braced};

use crate::types::delegate_component::DelegateEntry;
use crate::types::generics::TypeGenerics;

pub struct DelegateTable {
    pub table_ident: Ident,
    pub table_generics: TypeGenerics,
    pub entries: Punctuated<DelegateEntry, Comma>,
}

impl Parse for DelegateTable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let table_ident = input.parse()?;

        let table_generics: TypeGenerics = input.parse()?;

        let entries = {
            let content;
            braced!(content in input);

            Punctuated::parse_terminated(&content)?
        };

        Ok(Self {
            table_ident,
            table_generics,
            entries,
        })
    }
}
