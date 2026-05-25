use syn::Type;
use syn::parse::{Parse, ParseStream};
use syn::token::Colon;

use crate::types::delegate_component::{
    DelegateKey, DelegateValue, EvalDelegateEntry, EvalDelegateKey, EvalDelegateValue,
    EvaluatedDelegateEntry, ExtractInnerDelegateTables, InnerDelegateTable,
};

#[derive(Debug, Clone)]
pub struct NormalDelegateMapping {
    pub key: DelegateKey,
    pub colon: Colon,
    pub value: DelegateValue,
}

impl Parse for NormalDelegateMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        let colon = input.parse()?;
        let value = input.parse()?;

        Ok(Self { key, colon, value })
    }
}

impl EvalDelegateEntry for NormalDelegateMapping {
    fn eval(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        let keys = self.key.eval()?;
        let value_type = self.value.eval()?;

        let entries = keys
            .into_iter()
            .map(|key| EvaluatedDelegateEntry {
                table_type: table_type.clone(),
                generics: key.generics,
                key: key.key,
                value: value_type.clone(),
            })
            .collect();

        Ok(entries)
    }
}

impl ExtractInnerDelegateTables for NormalDelegateMapping {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable> {
        self.value.extract_inner_tables()
    }
}
