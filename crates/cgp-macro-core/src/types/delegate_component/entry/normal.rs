use syn::Type;
use syn::token::Colon;

use crate::types::delegate_component::{
    DelegateKey, DelegateValue, EvalDelegateEntry, EvaluatedDelegateEntry,
};

pub struct NormalDelegateEntry {
    pub key: DelegateKey,
    pub colon: Colon,
    pub value: DelegateValue,
}

impl EvalDelegateEntry for NormalDelegateEntry {
    fn eval(&self, context_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        todo!()
    }
}
