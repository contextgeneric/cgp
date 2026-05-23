use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Ident, braced};

use crate::types::delegate_component::DelegateEntry;
use crate::types::generics::TypeGenerics;
use crate::types::provider_struct::ProviderStruct;

pub trait ExtractInnerDelegateTables {
    fn inner_tables(&self) -> Vec<InnerDelegateTable>;
}

#[derive(Debug, Clone)]
pub struct InnerDelegateTable {
    pub table_ident: Ident,
    pub table_generics: TypeGenerics,
    pub entries: Punctuated<DelegateEntry, Comma>,
}

impl Parse for InnerDelegateTable {
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

impl InnerDelegateTable {
    pub fn to_provider_struct(&self) -> ProviderStruct {
        let ident = self.table_ident.clone();
        let generics = self.table_generics.generics.clone();

        ProviderStruct { ident, generics }
    }
}
