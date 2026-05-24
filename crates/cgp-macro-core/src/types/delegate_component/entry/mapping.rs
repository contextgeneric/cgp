use syn::Type;
use syn::parse::{Parse, ParseStream};

use crate::types::delegate_component::{
    DelegateMode, DirectDelegateEntry, EvalDelegateEntry, EvaluatedDelegateEntry,
    ExtractInnerDelegateTables, InnerDelegateTable, NormalDelegateEntry,
};

#[derive(Debug, Clone)]
pub enum DelegateMapping {
    Normal(NormalDelegateEntry),
    Direct(DirectDelegateEntry),
}

impl Parse for DelegateMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        let mode: DelegateMode = input.parse()?;
        let value = input.parse()?;

        let entry = match mode {
            DelegateMode::Normal(colon) => Self::Normal(NormalDelegateEntry { key, colon, value }),
            DelegateMode::Direct(arrow) => Self::Direct(DirectDelegateEntry { key, arrow, value }),
        };

        Ok(entry)
    }
}

impl EvalDelegateEntry for DelegateMapping {
    fn eval(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        match self {
            Self::Normal(entry) => entry.eval(table_type),
            Self::Direct(entry) => entry.eval(table_type),
        }
    }
}

impl ExtractInnerDelegateTables for DelegateMapping {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable> {
        match self {
            Self::Normal(entry) => entry.extract_inner_tables(),
            Self::Direct(entry) => entry.extract_inner_tables(),
        }
    }
}
