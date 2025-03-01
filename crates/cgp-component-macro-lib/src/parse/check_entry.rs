use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Colon, Comma};
use syn::{braced, bracketed, Type};

pub struct CheckComponents {
    pub context_type: Type,
    pub check_entries: CheckEntries,
}

pub struct CheckEntries {
    pub entries: Vec<(Type, Option<Type>)>,
}

struct CheckEntry {
    pub entries: Vec<(Type, Option<Type>)>,
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
        let component_types: Vec<Type> = if input.peek(Bracket) {
            let content;
            bracketed!(content in input);

            let types: Punctuated<Type, Comma> = Punctuated::parse_terminated(&content)?;
            Vec::from_iter(types)
        } else {
            let component_type: Type = input.parse()?;
            vec![component_type]
        };

        let component_params: Vec<Option<Type>> = if input.peek(Colon) {
            let _: Colon = input.parse()?;

            if input.peek(Bracket) {
                let content;
                bracketed!(content in input);

                let types: Punctuated<Type, Comma> = Punctuated::parse_terminated(&content)?;
                types.into_iter().map(Some).collect()
            } else {
                vec![Some(input.parse()?)]
            }
        } else {
            vec![None]
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
