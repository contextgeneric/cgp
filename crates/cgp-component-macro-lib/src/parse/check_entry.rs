use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Colon, Comma};
use syn::{bracketed, parse2, Type};

pub struct CheckEntries {
    pub entries: Vec<(Type, Type)>,
}

struct CheckEntry {
    pub entries: Vec<(Type, Type)>,
}

impl Parse for CheckEntries {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let check_entries: Punctuated<CheckEntry, Comma> = Punctuated::parse_terminated(input)?;

        let entries = check_entries
            .into_iter()
            .flat_map(|check_entry| check_entry.entries.into_iter())
            .collect();

        Ok(Self { entries })
    }
}

impl Parse for CheckEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let unit: Type = parse2(quote!(()))?;

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
                Vec::from_iter(types)
            } else {
                vec![input.parse()?]
            }
        } else {
            vec![unit.clone()]
        };

        let mut entries = Vec::new();

        for component_type in component_types.iter() {
            for component_param in component_params.iter() {
                entries.push((component_type.clone(), component_param.clone()))
            }
        }

        Ok(Self { entries })
    }
}
