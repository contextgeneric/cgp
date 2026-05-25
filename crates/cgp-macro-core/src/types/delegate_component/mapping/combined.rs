use syn::Type;
use syn::parse::{Parse, ParseStream};

use crate::types::delegate_component::{
    DelegateMode, DirectDelegateMapping, EvalDelegateEntries, EvaluatedDelegateEntry,
    ExtractInnerDelegateTables, InnerDelegateTable, NormalDelegateMapping, RedirectDelegateMapping,
};

#[derive(Debug, Clone)]
pub enum DelegateMapping {
    Normal(NormalDelegateMapping),
    Direct(DirectDelegateMapping),
    Redirect(RedirectDelegateMapping),
}

impl Parse for DelegateMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        let mode: DelegateMode = input.parse()?;

        let entry = match mode {
            DelegateMode::Normal(colon) => {
                let value = input.parse()?;
                Self::Normal(NormalDelegateMapping { key, colon, value })
            }
            DelegateMode::Direct(arrow) => {
                let value = input.parse()?;
                Self::Direct(DirectDelegateMapping { key, arrow, value })
            }
            DelegateMode::Redirect(arrow) => {
                let value = input.parse()?;
                Self::Redirect(RedirectDelegateMapping { key, arrow, value })
            }
        };

        Ok(entry)
    }
}

impl EvalDelegateEntries for DelegateMapping {
    fn eval_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        match self {
            Self::Normal(entry) => entry.eval_entries(table_type),
            Self::Direct(entry) => entry.eval_entries(table_type),
            Self::Redirect(entry) => entry.eval_entries(table_type),
        }
    }
}

impl ExtractInnerDelegateTables for DelegateMapping {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable> {
        match self {
            Self::Normal(entry) => entry.extract_inner_tables(),
            Self::Direct(entry) => entry.extract_inner_tables(),
            Self::Redirect(_entry) => Vec::new(),
        }
    }
}
