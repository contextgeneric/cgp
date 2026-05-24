use syn::parse::{Parse, ParseStream};
use syn::{Error, Type};

use crate::traits::PeekKeyword;
use crate::types::delegate_component::{
    EvalDelegateEntry, EvaluatedDelegateEntry, NamespaceDelegateEntry, OpenDelegateEntry,
};
use crate::types::keywords::{Namespace, Open};

#[derive(Debug, Clone)]
pub enum DelegateStatement {
    Namespace(NamespaceDelegateEntry),
    Open(OpenDelegateEntry),
}

impl Parse for DelegateStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek_keyword::<Namespace>() {
            let namespace_entry = input.parse()?;
            Ok(Self::Namespace(namespace_entry))
        } else if input.peek_keyword::<Open>() {
            let open_entry = input.parse()?;
            Ok(Self::Open(open_entry))
        } else {
            Err(Error::new(input.span(), "invalid delegate statement"))
        }
    }
}

impl EvalDelegateEntry for DelegateStatement {
    fn eval(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        match self {
            Self::Namespace(entry) => entry.eval(table_type),
            Self::Open(entry) => entry.eval(table_type),
        }
    }
}
