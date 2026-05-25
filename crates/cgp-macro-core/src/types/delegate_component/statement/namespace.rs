use syn::parse::{Parse, ParseStream};
use syn::token::Semi;
use syn::{Ident, Type, parse_quote};

use crate::types::delegate_component::{EvalDelegateEntry, EvaluatedDelegateEntry};
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

impl EvalDelegateEntry for NamespaceDelegateStatement {
    fn eval(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        let namespace_ident = if &self.ident == "default" {
            Ident::new("DefaultNamespace", self.ident.span())
        } else {
            self.ident.clone()
        };

        let key = parse_quote!(__Component__);
        let generics = parse_quote! {
            <__Component__: #namespace_ident< #table_type >>
        };

        let value = parse_quote! {
            < __Component__ as #namespace_ident< #table_type >>::Provider
        };

        let entry = EvaluatedDelegateEntry {
            table_type: table_type.clone(),
            generics,
            key,
            value,
        };

        Ok(vec![entry])
    }
}
