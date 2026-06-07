use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, ItemImpl, Type, braced, parse2};

use crate::types::delegate_component::DelegateEntries;
use crate::types::generics::TypeGenerics;
use crate::types::provider_struct::ProviderStruct;

pub trait ExtractInnerDelegateTables {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable>;
}

#[derive(Debug, Clone)]
pub struct InnerDelegateTable {
    pub table_ident: Ident,
    pub table_generics: TypeGenerics,
    pub entries: DelegateEntries,
}

impl Parse for InnerDelegateTable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let table_ident = input.parse()?;

        let table_generics: TypeGenerics = input.parse()?;

        let entries = {
            let body;
            braced!(body in input);

            body.parse()?
        };

        Ok(Self {
            table_ident,
            table_generics,
            entries,
        })
    }
}

impl InnerDelegateTable {
    pub fn build_table_type(&self) -> syn::Result<Type> {
        let ident = &self.table_ident;
        let type_generics = self.table_generics.split_for_impl().1;

        parse2(quote!( #ident #type_generics ))
    }

    pub fn build_table_struct(&self) -> ProviderStruct {
        let ident = self.table_ident.clone();
        let generics = self.table_generics.generics.clone();

        ProviderStruct { ident, generics }
    }

    pub fn build_impls(&self) -> syn::Result<Vec<ItemImpl>> {
        let table_type = self.build_table_type()?;
        self.entries.build_impls(&table_type, &self.table_generics)
    }
}

impl ExtractInnerDelegateTables for InnerDelegateTable {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable> {
        self.entries.extract_inner_tables()
    }
}
