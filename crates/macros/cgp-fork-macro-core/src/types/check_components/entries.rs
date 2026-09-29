use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::types::check_components::{CheckEntry, EvaluatedCheckEntry};

#[derive(Default)]
pub struct CheckEntries {
    pub entries: Punctuated<CheckEntry, Comma>,
}

impl CheckEntries {
    pub fn eval(&self) -> Vec<EvaluatedCheckEntry> {
        let mut evaluated_entries = Vec::new();

        for entry in &self.entries {
            evaluated_entries.extend(entry.eval());
        }

        evaluated_entries
    }
}

impl Parse for CheckEntries {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let entries: Punctuated<CheckEntry, Comma> = Punctuated::parse_terminated(input)?;

        Ok(Self { entries })
    }
}
