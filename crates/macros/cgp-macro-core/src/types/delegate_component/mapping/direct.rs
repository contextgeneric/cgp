use syn::Type;
use syn::token::RArrow;

use crate::exports::DelegateComponent;
use crate::parse_internal;
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

        let mut entries = Vec::new();

        for key in keys {
            let key_type = key.key;
            let mut generics = key.generics;

            let where_predicate = parse_internal! {
                #value_type: #DelegateComponent< #key_type >
            };

            generics
                .make_where_clause()
                .predicates
                .push(where_predicate);

            let direct_value_type = parse_internal! {
                < #value_type as #DelegateComponent< #key_type > >::Delegate
            };

            entries.push(EvaluatedDelegateEntry {
                table_type: table_type.clone(),
                generics,
                key: key_type,
                value: direct_value_type,
            });
        }

        Ok(entries)
    }
}

impl ExtractInnerDelegateTables for DirectDelegateMapping {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable> {
        self.value.extract_inner_tables()
    }
}
