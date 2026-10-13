use syn::spanned::Spanned;
use syn::token::RArrow;
use syn::{Type, WherePredicate};

use crate::exports::DelegateComponent;
use crate::functions::override_synthesized_span;
use crate::parse_internal;
use crate::types::delegate_component::{
    DelegateKey, DelegateValue, EvalDelegateEntries, EvalDelegateKey, EvalDelegateValue,
    EvaluatedDelegateEntry, ExtractInnerDelegateTables, InnerDelegateTable,
};

/// A `Key -> Value` mapping that forwards to the value's own entry for the key:
/// `Delegate` becomes `<Value as DelegateComponent<Key>>::Delegate`, with a
/// matching `Value: DelegateComponent<Key>` bound.
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
        let arrow_span = self.arrow.span();

        let mut entries = Vec::new();

        for key in keys {
            let key_type = key.key;
            let span = key.span;
            let mut generics = key.generics;

            // The bound and the projection mix the user's key and value with
            // synthesized tokens, so an error on either (the table lacking the key)
            // would span the whole block. Their synthesized tokens move onto the `->`
            // the user wrote, which no resolvable name shares, so the key and the
            // value stay navigable.
            let where_predicate: WherePredicate = override_synthesized_span(
                arrow_span,
                &parse_internal! {
                    #value_type: #DelegateComponent< #key_type >
                },
            )?;

            generics
                .make_where_clause()
                .predicates
                .push(where_predicate);

            let direct_value_type: Type = override_synthesized_span(
                arrow_span,
                &parse_internal! {
                    < #value_type as #DelegateComponent< #key_type > >::Delegate
                },
            )?;

            entries.push(EvaluatedDelegateEntry {
                table_type: table_type.clone(),
                generics,
                key: key_type,
                value: direct_value_type,
                span,
                delegate_span: Some(arrow_span),
                // The `-` alone, which does not contain the arrow's span the bounds sit on.
                params_span: Some(self.arrow.spans[0]),
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
