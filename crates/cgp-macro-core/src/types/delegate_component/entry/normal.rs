use syn::Type;
use syn::token::Colon;

use crate::types::delegate_component::{
    DelegateKey, DelegateValue, EvalDelegateEntry, EvalDelegateKey, EvaluatedDelegateEntry,
};

pub struct NormalDelegateEntry {
    pub key: DelegateKey,
    pub colon: Colon,
    pub value: DelegateValue,
}

impl EvalDelegateEntry for NormalDelegateEntry {
    fn eval(&self, _context_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        let keys = self.key.eval()?;
        let entries = Vec::new();

        for _key in keys {
            todo!()
        }

        Ok(entries)
    }
}
