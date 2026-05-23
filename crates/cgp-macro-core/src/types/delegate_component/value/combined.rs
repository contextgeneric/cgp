use syn::Type;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};

use crate::types::delegate_component::{
    DelegateValueWithInnerTable, EvalDelegateValue, ExtractInnerDelegateTables, InnerDelegateTable,
};

#[derive(Debug, Clone)]
pub enum DelegateValue {
    Type(Type),
    WithTable(DelegateValueWithInnerTable),
}

impl Parse for DelegateValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        if let Ok(value) = fork.parse::<DelegateValueWithInnerTable>() {
            input.advance_to(&fork);
            return Ok(Self::WithTable(value));
        }

        let ty: Type = input.parse()?;
        Ok(Self::Type(ty))
    }
}

impl EvalDelegateValue for DelegateValue {
    fn eval(&self) -> syn::Result<Type> {
        match self {
            Self::Type(ty) => Ok(ty.clone()),
            Self::WithTable(value) => value.eval(),
        }
    }
}

impl ExtractInnerDelegateTables for DelegateValue {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable> {
        match self {
            Self::Type(_) => Vec::new(),
            Self::WithTable(value) => value.extract_inner_tables(),
        }
    }
}
