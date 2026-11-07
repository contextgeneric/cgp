use syn::parse::{Parse, ParseStream};
use syn::{Ident, ItemImpl, Type, braced};

use crate::functions::parse_internal;
use crate::types::delegate_component::DelegateEntries;
use crate::types::empty_struct::EmptyStruct;
use crate::types::generics::TypeGenerics;

/// Collect every nested `UseDelegate<new Inner { .. }>` table reachable from a
/// construct, so `DelegateTable::eval` can emit each inner table's own items.
pub trait ExtractInnerDelegateTables {
    /// The nested tables.
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable>;
}

/// A nested table lifted out of a `UseDelegate<new Inner { .. }>` value; it is
/// evaluated exactly like a top-level table, keyed on its own identifier.
#[derive(Debug, Clone)]
pub struct InnerDelegateTable {
    /// The table's name.
    pub table_ident: Ident,
    /// The table's generic parameters.
    pub table_generics: TypeGenerics,
    /// The table's entries.
    pub entries: DelegateEntries,
}

impl Parse for InnerDelegateTable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let table_ident = input.parse()?;

        let table_generics: TypeGenerics = input.parse()?;

        let entries = {
            let body;
            braced!(body in input);

            body.parse()?
        };

        Ok(Self {
            table_ident,
            table_generics,
            entries,
        })
    }
}

impl InnerDelegateTable {
    /// Builds the table's type.
    pub fn build_table_type(&self) -> syn::Result<Type> {
        let ident = &self.table_ident;
        let type_generics = self.table_generics.split_for_impl().1;

        let ty = parse_internal!( #ident #type_generics );
        Ok(ty)
    }

    /// Builds the table's marker struct.
    pub fn build_table_struct(&self) -> EmptyStruct {
        let ident = self.table_ident.clone();
        let generics = self.table_generics.generics.clone();

        let doc = Some(format!("Wiring table `{ident}`."));
        EmptyStruct {
            ident,
            generics,
            doc,
        }
    }

    /// Builds the table's impls.
    pub fn build_impls(&self) -> syn::Result<Vec<ItemImpl>> {
        let table_type = self.build_table_type()?;
        self.entries.build_impls(&table_type, &self.table_generics)
    }
}

impl ExtractInnerDelegateTables for InnerDelegateTable {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable> {
        self.entries.extract_inner_tables()
    }
}
