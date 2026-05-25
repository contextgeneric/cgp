use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, For, Gt, In, Lt};
use syn::{Ident, Type, WhereClause, braced};

use crate::types::delegate_component::{
    DelegateMapping, EvalDelegateEntry, EvaluatedDelegateEntry,
};

#[derive(Debug, Clone)]
pub struct ForDelegateStatement {
    pub for_token: For,
    pub lt: Lt,
    pub key: Ident,
    pub comma: Comma,
    pub value: Ident,
    pub gt: Gt,
    pub in_token: In,
    pub namespace: Type,
    pub where_clause: Option<WhereClause>,
    pub mappings: Punctuated<DelegateMapping, Comma>,
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

impl EvalDelegateEntry for ForDelegateStatement {
    fn eval(&self, _table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        Ok(Vec::new())
    }
}
