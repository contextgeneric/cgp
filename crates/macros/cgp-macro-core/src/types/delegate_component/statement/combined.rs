use syn::parse::{Parse, ParseStream};
use syn::token::For;
use syn::{Error, Type};

use crate::traits::PeekKeyword;
use crate::types::delegate_component::{
    EvalDelegateEntries, EvaluatedDelegateEntry, ForDelegateStatement, NamespaceDelegateStatement,
    OpenDelegateStatement,
};
use crate::types::keywords::{Namespace, Open};

#[derive(Debug, Clone)]
pub enum DelegateStatement {
    Namespace(NamespaceDelegateStatement),
    Open(OpenDelegateStatement),
    For(Box<ForDelegateStatement>),
}

impl DelegateStatement {
    pub fn peek_statement(input: ParseStream) -> bool {
        input.peek_keyword::<Namespace>() || input.peek_keyword::<Open>() || input.peek(For)
    }
}

impl Parse for DelegateStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek_keyword::<Namespace>() {
            let namespace_entry = input.parse()?;
            Ok(Self::Namespace(namespace_entry))
        } else if input.peek_keyword::<Open>() {
            let open_entry = input.parse()?;
            Ok(Self::Open(open_entry))
        } else if input.peek(For) {
            let for_entry = input.parse()?;
            Ok(Self::For(for_entry))
        } else {
            Err(Error::new(input.span(), "invalid delegate statement"))
        }
    }
}

impl EvalDelegateEntries for DelegateStatement {
    fn eval_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        match self {
            Self::Namespace(entry) => entry.eval_entries(table_type),
            Self::Open(entry) => entry.eval_entries(table_type),
            Self::For(entry) => entry.eval_entries(table_type),
        }
    }
}
