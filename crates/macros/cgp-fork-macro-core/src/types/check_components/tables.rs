use syn::Item;
use syn::parse::{Parse, ParseStream};

use crate::types::check_components::CheckComponentsTable;

pub struct CheckComponentsTables {
    pub tables: Vec<CheckComponentsTable>,
}

impl CheckComponentsTables {
    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let mut items = Vec::new();

        for table in &self.tables {
            items.extend(table.to_items()?);
        }

        Ok(items)
    }
}

impl Parse for CheckComponentsTables {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut tables = Vec::new();

        while !input.is_empty() {
            let spec: CheckComponentsTable = input.parse()?;
            tables.push(spec);
        }

        Ok(Self { tables })
    }
}
