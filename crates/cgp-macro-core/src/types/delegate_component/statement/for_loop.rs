use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, For, Gt, In, Lt};
use syn::{Ident, Type, WhereClause, braced};

use crate::types::delegate_component::{
    EvalDelegateEntries, EvalDelegateKey, EvalDelegateValue, EvalForEntries,
    EvaluatedDelegateEntry, EvaluatedForEntry, NormalDelegateMapping,
    eval_delegate_entries_via_for,
};
use crate::types::ident::IdentWithTypeArgs;

#[derive(Debug, Clone)]
pub struct ForDelegateStatement {
    pub for_token: For,
    pub lt: Lt,
    pub key: Ident,
    pub comma: Comma,
    pub value: Ident,
    pub gt: Gt,
    pub in_token: In,
    pub namespace: IdentWithTypeArgs,
    pub where_clause: Option<WhereClause>,
    pub mappings: Punctuated<NormalDelegateMapping, Comma>,
}

impl Parse for ForDelegateStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let for_token = input.parse()?;
        let lt = input.parse()?;
        let key = input.parse()?;
        let comma = input.parse()?;
        let value = input.parse()?;
        let gt = input.parse()?;
        let in_token = input.parse()?;
        let namespace = input.parse()?;
        let where_clause = input.parse()?;

        let mappings = {
            let body;
            braced!(body in input);
            Punctuated::parse_terminated(&body)?
        };

        Ok(Self {
            for_token,
            lt,
            key,
            comma,
            value,
            gt,
            in_token,
            namespace,
            where_clause,
            mappings,
        })
    }
}

impl EvalForEntries for ForDelegateStatement {
    fn eval_for_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedForEntry>> {
        let mut entries = Vec::new();

        for mapping in &self.mappings {
            let keys = mapping.key.eval()?;
            let value_type = mapping.value.eval()?;

            for key in keys {
                let entry = EvaluatedForEntry {
                    generics: key.generics,
                    table_type: table_type.clone(),
                    for_key: self.key.clone(),
                    for_value: self.value.clone(),
                    namespace: self.namespace.clone(),
                    mapping_key: key.key,
                    mapping_value: value_type.clone(),
                };

                entries.push(entry);
            }
        }

        Ok(entries)
    }
}

impl EvalDelegateEntries for ForDelegateStatement {
    fn eval_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        eval_delegate_entries_via_for(self, table_type)
    }
}
