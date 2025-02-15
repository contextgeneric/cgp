use syn::parse::{Parse, ParseStream};

use crate::delegate_components::ast::DelegateEntriesAst;
use crate::parse::SimpleType;

pub struct DefinePresetAst {
    pub preset: SimpleType,
    pub delegate_entries: DelegateEntriesAst,
}

impl Parse for DefinePresetAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let preset = input.parse()?;

        let delegate_entries: DelegateEntriesAst = input.parse()?;

        Ok(Self {
            preset,
            delegate_entries,
        })
    }
}
