use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Bracket, Colon, Comma};
use syn::{braced, bracketed, Type};

pub struct CheckComponents {
    pub context_type: Type,
    pub check_entries: CheckEntries,
}

pub struct CheckEntries {
    pub entries: Vec<CheckEntry>,
}

pub struct CheckEntry {
    pub component_type: Type,
    pub component_params: Option<Type>,
    pub span: Span,
}

struct ParseCheckEntries {
    pub entries: Vec<CheckEntry>,
}

impl Parse for CheckComponents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let context_type: Type = input.parse()?;

        let content;
        braced!(content in input);

        let entries: CheckEntries = content.parse()?;

        Ok(Self {
            context_type,
            check_entries: entries,
        })
    }
}

impl Parse for CheckEntries {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let check_entries: Punctuated<ParseCheckEntries, Comma> =
            Punctuated::parse_terminated(input)?;

        let entries = check_entries
            .into_iter()
            .flat_map(|check_entry| check_entry.entries.into_iter())
            .collect();

        Ok(Self { entries })
    }
}

impl Parse for ParseCheckEntries {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_types: Vec<Type> = if input.peek(Bracket) {
            let content;
            bracketed!(content in input);

            let types: Punctuated<Type, Comma> = Punctuated::parse_terminated(&content)?;
            Vec::from_iter(types)
        } else {
            let component_type: Type = input.parse()?;
            vec![component_type]
        };

        let component_params: Vec<Type> = if input.peek(Colon) {
            let _: Colon = input.parse()?;

            if input.peek(Bracket) {
                let content;
                bracketed!(content in input);

                let types: Punctuated<Type, Comma> = Punctuated::parse_terminated(&content)?;
                types.into_iter().collect()
            } else {
                vec![input.parse()?]
            }
        } else {
            vec![]
        };

        let mut entries = Vec::new();

        let component_types_count = component_types.len();

        for component_type in component_types.iter() {
            if component_params.is_empty() {
                entries.push(CheckEntry {
                    component_type: component_type.clone(),
                    component_params: None,
                    span: component_type.span(),
                })
            } else {
                let component_params_count = component_params.len();

                for component_param in component_params.iter() {
                    let span = if component_types_count >= component_params_count {
                        component_type.span()
                    } else {
                        component_param.span()
                    };

                    entries.push(CheckEntry {
                        component_type: component_type.clone(),
                        component_params: Some(component_param.clone()),
                        span,
                    })
                }
            }
        }

        Ok(Self { entries })
    }
}
