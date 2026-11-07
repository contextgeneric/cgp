use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Error, Ident};

use crate::types::check_components::{
    CheckComponentsTable, CheckEntries, derive_check_trait_ident,
};
use crate::types::delegate_and_check_components::ToKeysWithCheckParams;
use crate::types::delegate_component::DelegateTable;

/// The parsed `delegate_and_check_components!` input.
pub struct ItemDelegateAndCheckComponents {
    /// The wiring table.
    pub table: DelegateTable,
}

impl ItemDelegateAndCheckComponents {
    /// Builds the check table for the wiring.
    pub fn to_check_components(&self) -> syn::Result<CheckComponentsTable> {
        let trait_name = self.check_trait_ident()?;
        let check_entries = self.to_check_entries()?;

        let check_table = CheckComponentsTable {
            check_providers: None,
            impl_generics: self.table.impl_generics.clone(),
            trait_name,
            context_type: self.table.table_type.clone(),
            where_clause: None,
            check_entries,
        };

        Ok(check_table)
    }

    /// Builds the check entries for the wiring.
    pub fn to_check_entries(&self) -> syn::Result<CheckEntries> {
        let keys = self.table.entries.to_keys_with_check_params()?;

        let mut entries = CheckEntries::default();

        for key in keys {
            entries.entries.extend(key.to_check_entries().entries);
        }

        Ok(entries)
    }

    /// The check trait's name.
    pub fn check_trait_ident(&self) -> syn::Result<Ident> {
        let attributes = &self.table.attributes;

        if attributes.is_empty() {
            derive_check_trait_ident(&self.table.table_type, "__CanUse")
        } else if attributes.len() > 1 {
            Err(Error::new(
                attributes[1].span(),
                "Expected exactly one attribute for the check trait name",
            ))
        } else {
            let attribute = &attributes[0];
            if !attribute.path().is_ident("check_trait") {
                return Err(syn::Error::new(
                    attribute.span(),
                    "Expected `#[check_trait]` attribute for specifying the check trait name",
                ));
            }

            let ident = attribute.parse_args()?;

            Ok(ident)
        }
    }
}

impl Parse for ItemDelegateAndCheckComponents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let table = input.parse()?;

        Ok(Self { table })
    }
}
