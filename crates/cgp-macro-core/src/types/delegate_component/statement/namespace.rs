use syn::parse::{Parse, ParseStream};
use syn::token::Semi;
use syn::{Generics, Ident, Type, parse_quote};

use crate::types::delegate_component::{
    EvalDelegateEntries, EvalForEntry, EvaluatedDelegateEntry, EvaluatedForEntry,
    eval_delegate_entries_via_for,
};
use crate::types::generics::TypeGenerics;
use crate::types::keyword::Keyword;
use crate::types::keywords::Namespace;

#[derive(Debug, Clone)]
pub struct NamespaceDelegateStatement {
    pub namespace: Keyword<Namespace>,
    pub ident: Ident,
    pub semi: Semi,
}

impl Parse for NamespaceDelegateStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let namespace = input.parse()?;
        let ident = input.parse()?;
        let semi = input.parse()?;

        Ok(Self {
            namespace,
            ident,
            semi,
        })
    }
}

impl EvalForEntry for NamespaceDelegateStatement {
    fn eval_for(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedForEntry>> {
        let namespace_ident = if &self.ident == "default" {
            Ident::new("DefaultNamespace", self.ident.span())
        } else {
            self.ident.clone()
        };

        let entry = EvaluatedForEntry {
            generics: Generics::default(),
            table_type: table_type.clone(),
            for_key: parse_quote!(__Key__),
            for_value: parse_quote!(__Value__),
            mapping_key: parse_quote!(__Key__),
            mapping_value: parse_quote!(__Value__),
            namespace_ident,
            namespace_generics: TypeGenerics::default(),
        };

        Ok(vec![entry])
    }
}

impl EvalDelegateEntries for NamespaceDelegateStatement {
    fn eval_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        eval_delegate_entries_via_for(self, table_type)
    }
}
