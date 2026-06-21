use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Bracket, Colon, Comma};
use syn::{Type, bracketed};

use crate::types::check_components::{CheckEntry, TypeWithGenerics};
use crate::types::generics::ImplGenerics;

pub struct CheckEntries {
    pub entries: Vec<CheckEntry>,
}

struct ParseCheckEntries {
    pub entries: Vec<CheckEntry>,
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

        let component_params: Vec<TypeWithGenerics> = if input.peek(Colon) {
            let _: Colon = input.parse()?;

            if input.peek(Bracket) {
                let content;
                bracketed!(content in input);

                let types: Punctuated<TypeWithGenerics, Comma> =
                    Punctuated::parse_terminated(&content)?;
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
                    generics: ImplGenerics::default(),
                })
            } else {
                let component_params_count = component_params.len();

                for component_param in component_params.iter() {
                    let component_param_type = &component_param.ty;
                    let component_param_generics = &component_param.generics;

                    let span = if component_types_count >= component_params_count {
                        component_type.span()
                    } else {
                        component_param_type.span()
                    };

                    entries.push(CheckEntry {
                        component_type: component_type.clone(),
                        component_params: Some(component_param_type.clone()),
                        span,
                        generics: component_param_generics.clone(),
                    })
                }
            }
        }

        Ok(Self { entries })
    }
}
