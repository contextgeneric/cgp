use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Plus};

use crate::delegate_components::ast::DelegateEntriesAst;
use crate::parse::SimpleType;

pub struct DefinePresetAst {
    pub preset: SimpleType,
    pub parents: Punctuated<SimpleType, Plus>,
    pub delegate_entries: DelegateEntriesAst,
}

impl Parse for DefinePresetAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let preset = input.parse()?;

        let parents = if input.peek(Colon) {
            let _: Colon = input.parse()?;
            Punctuated::parse_separated_nonempty(input)?
        } else {
            Punctuated::default()
        };

        let delegate_entries: DelegateEntriesAst = input.parse()?;

        Ok(Self {
            preset,
            parents,
            delegate_entries,
        })
    }
}
