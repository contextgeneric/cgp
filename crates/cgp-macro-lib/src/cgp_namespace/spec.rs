use syn::parse::{Parse, ParseStream};
use syn::token::{At, Colon};
use syn::{Ident, Type, braced};

use crate::parse::ComponentPaths;

pub struct NamespaceSpec {
    pub namespace_ident: Ident,
    pub parent_namespace_ident: Option<Ident>,
    pub entries: Vec<NamespaceEntry>,
}

pub struct NamespaceEntry {
    pub source: Type,
    pub target: Type,
}

impl Parse for NamespaceSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let namespace_ident: Ident = input.parse()?;

        let parent_namespace_ident: Option<Ident> = if input.peek(Colon) {
            let _: Colon = input.parse()?;
            let ident = input.parse()?;
            Some(ident)
        } else {
            None
        };

        let content;
        braced!(content in input);

        let entries = parse_namespace_entries(&content)?;

        Ok(NamespaceSpec {
            namespace_ident,
            parent_namespace_ident,
            entries,
        })
    }
}

fn parse_namespace_entries(input: ParseStream) -> syn::Result<Vec<NamespaceEntry>> {
    let mut entries = Vec::new();

    while !input.is_empty() {
        if input.peek(At) {
            let _: At = input.parse()?;

            let paths: ComponentPaths = input.parse()?;

            let _: Colon = input.parse()?;

            if input.peek(At) {}
        } else {
        }
    }

    Ok(entries)
}
