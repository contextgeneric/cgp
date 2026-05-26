use syn::{Generics, Ident, Type, parse_quote};

use crate::types::delegate_component::{EvalForEntry, EvaluatedForEntry};
use crate::types::generics::TypeGenerics;

#[derive(Debug, Clone)]
pub struct InheritNamespaceStatement {
    pub ident: Ident,
    pub type_generics: TypeGenerics,
}

impl EvalForEntry for InheritNamespaceStatement {
    fn eval_for_entry(&self, table_type: &Type) -> syn::Result<EvaluatedForEntry> {
        let namespace_ident = &self.ident;
        let namespace_generics = &self.type_generics;

        let mut generics = Generics::default();
        generics.make_where_clause().predicates.push(parse_quote! {
            __Key__: #namespace_ident #namespace_generics
        });

        let for_entry = EvaluatedForEntry {
            generics,
            table_type: table_type.clone(),
            for_key: parse_quote!(__Key__),
            for_value: parse_quote!(__Value__),
            mapping_key: parse_quote!(__Key__),
            mapping_value: parse_quote!(__Value__),
            namespace_ident: self.ident.clone(),
            namespace_generics: self.type_generics.clone(),
        };

        Ok(for_entry)
    }
}
