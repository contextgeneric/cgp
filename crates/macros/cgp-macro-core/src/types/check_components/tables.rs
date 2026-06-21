use syn::parse::{Parse, ParseStream};

use crate::types::check_components::CheckComponentsTable;

pub struct CheckComponentsTables {
    pub specs: Vec<CheckComponentsTable>,
}

impl Parse for CheckComponentsTables {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut specs = Vec::new();

        while !input.is_empty() {
            let spec: CheckComponentsTable = input.parse()?;
            specs.push(spec);
        }

        Ok(Self { specs })
    }
}
