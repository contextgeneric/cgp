use syn::Type;
use syn::parse::{Parse, ParseStream};

use crate::traits::PeekKeyword;
use crate::types::delegate_component::{
    DelegateMode, DirectDelegateEntry, EvalDelegateEntry, EvaluatedDelegateEntry, Namespace,
    NamespaceDelegateEntry, NormalDelegateEntry,
};

pub enum DelegateEntry {
    Normal(NormalDelegateEntry),
    Direct(DirectDelegateEntry),
    Namespace(NamespaceDelegateEntry),
}

impl Parse for DelegateEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek_keyword::<Namespace>() {
            let namespace_entry = input.parse()?;
            Ok(Self::Namespace(namespace_entry))
        } else {
            let key = input.parse()?;
            let mode: DelegateMode = input.parse()?;
            let value = input.parse()?;

            let entry = match mode {
                DelegateMode::Normal(colon) => {
                    Self::Normal(NormalDelegateEntry { key, colon, value })
                }
                DelegateMode::Direct(arrow) => {
                    Self::Direct(DirectDelegateEntry { key, arrow, value })
                }
            };

            Ok(entry)
        }
    }
}

impl EvalDelegateEntry for DelegateEntry {
    fn eval(&self, context_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        match self {
            Self::Normal(entry) => entry.eval(context_type),
            Self::Direct(entry) => entry.eval(context_type),
            Self::Namespace(entry) => entry.eval(context_type),
        }
    }
}
