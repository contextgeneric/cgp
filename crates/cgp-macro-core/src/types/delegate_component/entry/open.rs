use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Semi};
use syn::{Type, parse_quote};

use crate::exports::{Nil, PathCons, RedirectLookup};
use crate::types::delegate_component::{EvalDelegateEntry, EvaluatedDelegateEntry};
use crate::types::keyword::Keyword;
use crate::types::keywords::Open;

#[derive(Debug, Clone)]
pub struct OpenDelegateEntry {
    pub open: Keyword<Open>,
    pub components: Punctuated<Type, Comma>,
    pub semi: Semi,
}

impl Parse for OpenDelegateEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let open = input.parse()?;

        let components: Punctuated<Type, Comma> = Punctuated::parse_separated_nonempty(input)?;
        let semi = input.parse()?;

        Ok(Self {
            open,
            components,
            semi,
        })
    }
}

impl EvalDelegateEntry for OpenDelegateEntry {
    fn eval(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        let mut entries = Vec::new();

        for component in &self.components {
            let value: Type = parse_quote! {
                #RedirectLookup<
                    #table_type,
                    #PathCons<#component, #Nil>,
                >
            };

            let key = component.clone();

            entries.push(EvaluatedDelegateEntry {
                table_type: table_type.clone(),
                generics: Default::default(),
                key,
                value,
            })
        }

        Ok(entries)
    }
}
