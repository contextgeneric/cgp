use syn::parse::{Parse, ParseStream};
use syn::{Ident, Type};

use crate::macro_core::traits::PeekKeyword;
use crate::macro_core::types::delegate_component::{
    DelegateMode, DirectDelegateMapping, EvalDelegateEntries, EvaluatedDelegateEntry,
    ExtractInnerDelegateTables, GetterDelegateShorthand, InnerDelegateTable, NormalDelegateMapping,
    PresetDelegateEntry, RedirectDelegateMapping, TypeDelegateShorthand,
};
use crate::macro_core::types::keywords::{Getter, Preset};

/// One `Key OP Value` entry, its variant chosen by the operator: `:` is Normal,
/// `->` is Direct, `=>` is Redirect.
#[derive(Debug, Clone)]
pub enum DelegateMapping {
    Normal(NormalDelegateMapping),
    Direct(DirectDelegateMapping),
    Redirect(RedirectDelegateMapping),
    TypeShorthand(TypeDelegateShorthand),
    GetterShorthand(GetterDelegateShorthand),
    Preset(PresetDelegateEntry),
}

impl Parse for DelegateMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![type]) {
            return Ok(Self::TypeShorthand(input.parse()?));
        }

        if peek_bare_keyword::<Getter>(input) {
            return Ok(Self::GetterShorthand(input.parse()?));
        }

        if peek_bare_keyword::<Preset>(input) {
            return Ok(Self::Preset(input.parse()?));
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
            // A preset's keys are not known here; `build_preset_invocations`
            // emits the `with_components!` call that expands them.
            Self::Preset(_) => Ok(Vec::new()),
        }
    }
}

impl ExtractInnerDelegateTables for DelegateMapping {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable> {
        match self {
            Self::Normal(entry) => entry.extract_inner_tables(),
            Self::Direct(entry) => entry.extract_inner_tables(),
            Self::Redirect(_)
            | Self::TypeShorthand(_)
            | Self::GetterShorthand(_)
            | Self::Preset(_) => Vec::new(),
        }
    }
}

/// `getter name` / `preset Path` rather than a key of that name followed by an
/// operator. A following `:`, `->`, or `=>` keeps the explicit `Key OP Value` form.
fn peek_bare_keyword<K>(input: ParseStream) -> bool
where
    K: crate::macro_core::traits::IsKeyword,
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
