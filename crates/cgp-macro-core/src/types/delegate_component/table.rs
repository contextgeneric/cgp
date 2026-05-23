use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Ident, braced};

use crate::traits::PeekKeyword;
use crate::types::delegate_component::DelegateEntry;
use crate::types::generics::ImplGenerics;
use crate::types::keyword::Keyword;
use crate::types::keywords::New;

pub struct DelegateTable {
    pub impl_generics: ImplGenerics,
    pub new: Option<Keyword<New>>,
    pub table_type: Ident,
    pub entries: Punctuated<DelegateEntry, Comma>,
}

impl Parse for DelegateTable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let impl_generics = input.parse()?;

        let new = if input.peek_keyword::<New>() {
            Some(input.parse()?)
        } else {
            None
        };

        let table_type = input.parse()?;

        let entries = {
            let content;
            braced!(content in input);

            Punctuated::parse_terminated(&content)?
        };

        Ok(Self {
            impl_generics,
            new,
            table_type,
            entries,
        })
    }
}
