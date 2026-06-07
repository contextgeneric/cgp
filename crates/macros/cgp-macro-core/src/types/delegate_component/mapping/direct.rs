use syn::token::RArrow;
use syn::{Type, parse_quote};

use crate::exports::DelegateComponent;
use crate::types::delegate_component::{
    DelegateKey, DelegateValue, EvalDelegateEntries, EvalDelegateKey, EvalDelegateValue,
    EvaluatedDelegateEntry, ExtractInnerDelegateTables, InnerDelegateTable,
};

#[derive(Debug, Clone)]
pub struct DirectDelegateMapping {
    pub key: DelegateKey,
    pub arrow: RArrow,
    pub value: DelegateValue,
}

impl EvalDelegateEntries for DirectDelegateMapping {
    fn eval_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
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
                    table_type: table_type.clone(),
                    generics,
                    key: key_type,
                    value: direct_value_type,
                }
            })
            .collect();

        Ok(entries)
    }
}

impl ExtractInnerDelegateTables for DirectDelegateMapping {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable> {
        self.value.extract_inner_tables()
    }
}
