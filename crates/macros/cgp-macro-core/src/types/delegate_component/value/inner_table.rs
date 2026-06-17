use syn::parse::{Parse, ParseStream};
use syn::token::{Gt, Lt};
use syn::{Ident, Type};

use crate::parse_internal;
use crate::types::delegate_component::{
    EvalDelegateValue, ExtractInnerDelegateTables, InnerDelegateTable,
};
use crate::types::keyword::Keyword;
use crate::types::keywords::New;

#[derive(Debug, Clone)]
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

        let ty = parse_internal!( #wrapper_ident < #struct_ident #struct_generics > );
        Ok(ty)
    }
}

impl ExtractInnerDelegateTables for DelegateValueWithInnerTable {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable> {
        let mut inner_tables = self.inner_table.extract_inner_tables();
        inner_tables.push(self.inner_table.clone());

        inner_tables
    }
}
