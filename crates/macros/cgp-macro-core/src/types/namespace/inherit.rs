use syn::{Generics, Ident, Type};

use crate::parse_internal;
use crate::types::delegate_component::{EvalForEntry, EvaluatedForEntry};
use crate::types::ident::IdentWithTypeArgs;

#[derive(Debug, Clone)]
pub struct InheritNamespaceStatement {
    pub namespace: IdentWithTypeArgs,
    pub local_table_ident: Ident,
}

impl EvalForEntry for InheritNamespaceStatement {
    fn eval_for_entry(&self, table_type: &Type) -> syn::Result<EvaluatedForEntry> {
        let local_table_ident = &self.local_table_ident;

        let mut namespace_constraint = self.namespace.clone();
        namespace_constraint
            .type_args
            .make_args()
            .push(parse_internal!(#local_table_ident));

        let mut generics = Generics::default();
        generics
            .make_where_clause()
            .predicates
            .push(parse_internal! {
                __Key__: #namespace_constraint
            });

        let for_entry = EvaluatedForEntry {
            generics,
            table_type: table_type.clone(),
            for_key: parse_internal!(__Key__),
            for_value: parse_internal!(__Value__),
            mapping_key: parse_internal!(__Key__),
            mapping_value: parse_internal!(__Value__),
            namespace: self.namespace.clone(),
        };

        Ok(for_entry)
    }
}
