use cgp_macro_core::types::{PathHeadOrType, UniPath};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Comma};
use syn::{Ident, braced};

pub struct NamespaceSpec {
    pub namespace_ident: Ident,
    pub parent_namespace_ident: Option<Ident>,
    pub entries: Punctuated<NamespaceEntry, Comma>,
}

pub struct NamespaceEntry {
    pub keys: PathHeadOrType,
    pub value: UniPath,
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

        let entries = Punctuated::parse_terminated(&content)?;

        Ok(NamespaceSpec {
            namespace_ident,
            parent_namespace_ident,
            entries,
        })
    }
}

impl Parse for NamespaceEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let keys = input.parse()?;

        let _: Colon = input.parse()?;

        let value = input.parse()?;

        Ok(Self { keys, value })
    }
}
