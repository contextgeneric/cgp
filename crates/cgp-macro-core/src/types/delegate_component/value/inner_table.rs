use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Gt, Lt};
use syn::{Ident, Type, braced, parse_quote};

use crate::types::delegate_component::{DelegateEntry, EvalDelegateValue};
use crate::types::generics::TypeGenerics;
use crate::types::keyword::Keyword;
use crate::types::keywords::New;
use crate::types::provider_struct::ProviderStruct;

pub struct DelegateValueWithInnerTable {
    pub new: Keyword<New>,
    pub wrapper_ident: Ident,
    pub inner_table: InnerDelegateTable,
}

impl Parse for DelegateValueWithInnerTable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let wrapper_ident = input.parse()?;

        let _: Lt = input.parse()?;

        let new = input.parse()?;

        let inner_table = input.parse()?;

        let _: Gt = input.parse()?;

        Ok(Self {
            new,
            wrapper_ident,
            inner_table,
        })
    }
}

impl EvalDelegateValue for DelegateValueWithInnerTable {
    fn eval(&self) -> syn::Result<Type> {
        let wrapper_ident = &self.wrapper_ident;
        let struct_ident = &self.inner_table.table_ident;
        let struct_generics = &self.inner_table.table_generics;

        let ty = parse_quote!( #wrapper_ident < #struct_ident #struct_generics > );
        Ok(ty)
    }
}

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
