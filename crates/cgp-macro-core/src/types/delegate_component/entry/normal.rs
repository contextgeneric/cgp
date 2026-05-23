use syn::Type;
use syn::token::Colon;

use crate::types::delegate_component::{
    DelegateKey, DelegateValue, EvalDelegateEntry, EvalDelegateKey, EvalDelegateValue,
    EvaluatedDelegateEntry,
};

pub struct NormalDelegateEntry {
    pub key: DelegateKey,
    pub colon: Colon,
    pub value: DelegateValue,
}

impl EvalDelegateEntry for NormalDelegateEntry {
    fn eval(&self, context_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        let keys = self.key.eval()?;
        let value_type = self.value.eval()?;

        let entries = keys
            .into_iter()
            .map(|key| EvaluatedDelegateEntry {
                table_type: context_type.clone(),
                generics: key.generics,
                key: key.key,
                value: value_type.clone(),
            })
            .collect();

        Ok(entries)
    }
}
