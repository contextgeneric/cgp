use syn::parse::{Parse, ParseStream};
use syn::token::{Gt, Lt};
use syn::{Ident, Type, parse_quote};

use crate::types::delegate_component::{DelegateTable, EvalDelegateValue};
use crate::types::keyword::Keyword;
use crate::types::keywords::New;

pub struct DelegateValueWithInnerTable {
    pub new: Keyword<New>,
    pub wrapper_ident: Ident,
    pub inner_table: DelegateTable,
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
