use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Brace, Comma, Semi};
use syn::{Type, braced};

use crate::exports::{Nil, PathCons, RedirectLookup};
use crate::parse_internal;
use crate::types::delegate_component::{EvalDelegateEntries, EvaluatedDelegateEntry};
use crate::types::keyword::Keyword;
use crate::types::keywords::Open;

/// The `open { A, B };` header — braces optional when opening a single
/// component (`open A;`). Each listed component is wired to a
/// `RedirectLookup` rooted at the component name in the context's own table, so
/// the `@Component.Key` mappings that follow dispatch on the redirect path.
#[derive(Debug, Clone)]
pub struct OpenDelegateStatement {
    pub open: Keyword<Open>,
    pub components: Punctuated<Type, Comma>,
    pub semi: Semi,
}

impl Parse for OpenDelegateStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let open = input.parse()?;

        let components: Punctuated<Type, Comma> = if input.peek(Brace) {
            let body;
            braced!(body in input);

            Punctuated::parse_terminated(&body)?
        } else {
            // Braceless single-component form `open A;`. Opening several
            // components at once still requires the braced list.
            let component: Type = input.parse()?;
            Punctuated::from_iter([component])
        };

        let semi = input.parse()?;

        Ok(Self {
            open,
            components,
            semi,
        })
    }
}

impl EvalDelegateEntries for OpenDelegateStatement {
    fn eval_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        let mut entries = Vec::new();

        for component in &self.components {
            let value: Type = parse_internal! {
                #RedirectLookup<
                    #table_type,
                    #PathCons<#component, #Nil>,
                >
            };

            let key = component.clone();

            entries.push(EvaluatedDelegateEntry {
                table_type: table_type.clone(),
                generics: Default::default(),
                span: component.span(),
                key,
                value,
            })
        }

        Ok(entries)
    }
}
