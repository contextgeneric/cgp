use syn::parse::{Parse, ParseStream};
use syn::{Ident, Type};

use crate::traits::PeekKeyword;
use crate::types::delegate_component::{
    DelegateMode, DirectDelegateMapping, EvalDelegateEntries, EvaluatedDelegateEntry,
    ExtractInnerDelegateTables, GetterDelegateShorthand, InnerDelegateTable, NormalDelegateMapping,
    RedirectDelegateMapping, TypeDelegateShorthand,
};
use crate::types::keywords::Getter;

/// One `Key OP Value` entry, its variant chosen by the operator: `:` is Normal,
/// `->` is Direct, `=>` is Redirect.
#[derive(Debug, Clone)]
pub enum DelegateMapping {
    Normal(NormalDelegateMapping),
    Direct(DirectDelegateMapping),
    Redirect(RedirectDelegateMapping),
    TypeShorthand(TypeDelegateShorthand),
    GetterShorthand(GetterDelegateShorthand),
}

impl Parse for DelegateMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![type]) {
            return Ok(Self::TypeShorthand(input.parse()?));
        }

        if peek_bare_keyword::<Getter>(input) {
            return Ok(Self::GetterShorthand(input.parse()?));
        }

        let key = input.parse()?;
        let mode: DelegateMode = input.parse()?;

        let entry = match mode {
            DelegateMode::Normal(colon) => {
                let value = input.parse()?;
                Self::Normal(NormalDelegateMapping { key, colon, value })
            }
            DelegateMode::Direct(arrow) => {
                let value = input.parse()?;
                Self::Direct(DirectDelegateMapping { key, arrow, value })
            }
            DelegateMode::Redirect(arrow) => {
                let value = input.parse()?;
                Self::Redirect(RedirectDelegateMapping { key, arrow, value })
            }
        };

        Ok(entry)
    }
}

impl EvalDelegateEntries for DelegateMapping {
    fn eval_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        match self {
            Self::Normal(entry) => entry.eval_entries(table_type),
            Self::Direct(entry) => entry.eval_entries(table_type),
            Self::Redirect(entry) => entry.eval_entries(table_type),
            Self::TypeShorthand(entry) => entry.eval_entries(table_type),
            Self::GetterShorthand(entry) => entry.eval_entries(table_type),
        }
    }
}

impl ExtractInnerDelegateTables for DelegateMapping {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable> {
        match self {
            Self::Normal(entry) => entry.extract_inner_tables(),
            Self::Direct(entry) => entry.extract_inner_tables(),
            Self::Redirect(_) | Self::TypeShorthand(_) | Self::GetterShorthand(_) => Vec::new(),
        }
    }
}

/// `getter name` rather than a key of that name followed by an operator.
/// A following `:`, `->`, or `=>` keeps the explicit `Key OP Value` form.
fn peek_bare_keyword<K>(input: ParseStream) -> bool
where
    K: crate::traits::IsKeyword,
{
    if !input.peek_keyword::<K>() {
        return false;
    }

    let fork = input.fork();
    if fork.parse::<Ident>().is_err() {
        return false;
    }

    !fork.peek(syn::Token![:]) && !fork.peek(syn::Token![->]) && !fork.peek(syn::Token![=>])
}
