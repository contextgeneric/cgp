use syn::parse::{Parse, ParseStream};
use syn::token::Lt;
use syn::{Generics, Ident};

use crate::delegate_components::ast::DelegateEntriesAst;

pub struct DefinePresetAst {
    pub preset_ident: Ident,
    pub preset_generics: Generics,
    pub delegate_entries: DelegateEntriesAst,
}

impl Parse for DefinePresetAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let preset_ident: Ident = input.parse()?;

        let preset_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let delegate_entries: DelegateEntriesAst = input.parse()?;

        Ok(Self {
            preset_ident,
            preset_generics,
            delegate_entries,
        })
    }
}
