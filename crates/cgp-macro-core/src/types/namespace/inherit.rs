use syn::{Generics, Ident, Type, parse_quote};

use crate::types::delegate_component::{EvalForEntry, EvaluatedForEntry};
use crate::types::generics::TypeGenerics;

#[derive(Debug, Clone)]
pub struct InheritNamespaceStatement {
    pub ident: Ident,
    pub type_generics: TypeGenerics,
    pub local_table_ident: Ident,
}

impl EvalForEntry for InheritNamespaceStatement {
    fn eval_for_entry(&self, table_type: &Type) -> syn::Result<EvaluatedForEntry> {
        let namespace_ident = self.ident.clone();
        let local_table_ident = &self.local_table_ident;

        let mut namespace_where_generics = self.type_generics.clone();

        namespace_where_generics
            .params
            .push(parse_quote!(#local_table_ident));

        let mut generics = Generics::default();
        generics.make_where_clause().predicates.push(parse_quote! {
            __Key__: #namespace_ident #namespace_where_generics
        });

        let for_entry = EvaluatedForEntry {
            generics,
            table_type: table_type.clone(),
            for_key: parse_quote!(__Key__),
            for_value: parse_quote!(__Value__),
            mapping_key: parse_quote!(__Key__),
            mapping_value: parse_quote!(__Value__),
            namespace_ident,
            namespace_generics: self.type_generics.clone(),
        };

        Ok(for_entry)
    }
}
