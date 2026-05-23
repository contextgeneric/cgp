use syn::token::RArrow;
use syn::{Type, parse_quote};

use crate::exports::DelegateComponent;
use crate::types::delegate_component::{
    DelegateKey, DelegateValue, EvalDelegateEntry, EvalDelegateKey, EvalDelegateValue,
    EvaluatedDelegateEntry,
};

pub struct DirectDelegateEntry {
    pub key: DelegateKey,
    pub arrow: RArrow,
    pub value: DelegateValue,
}

impl EvalDelegateEntry for DirectDelegateEntry {
    fn eval(&self, context_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        let keys = self.key.eval()?;
        let value_type = self.value.eval()?;

        let entries = keys
            .into_iter()
            .map(|key| {
                let key_type = key.key;
                let mut generics = key.generics;

                let where_predicate = parse_quote! {
                    #value_type: #DelegateComponent< #key_type >
                };

                generics
                    .make_where_clause()
                    .predicates
                    .push(where_predicate);

                let direct_value_type = parse_quote! {
                    < #value_type as #DelegateComponent< #key_type > >::Delegate
                };

                EvaluatedDelegateEntry {
                    table_type: context_type.clone(),
                    generics,
                    key: key_type,
                    value: direct_value_type,
                }
            })
            .collect();

        Ok(entries)
    }
}
