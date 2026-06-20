use syn::{Generics, Ident, Type};

use crate::parse_internal;
use crate::types::delegate_component::{EvalDelegateEntry, EvaluatedDelegateEntry};
use crate::types::ident::PathWithTypeArgs;

pub trait EvalForEntries {
    fn eval_for_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedForEntry>>;
}

pub trait EvalForEntry {
    fn eval_for_entry(&self, table_type: &Type) -> syn::Result<EvaluatedForEntry>;
}

pub struct EvaluatedForEntry {
    pub generics: Generics,
    pub table_type: Type,
    pub for_key: Ident,
    pub for_value: Ident,
    pub namespace: PathWithTypeArgs,
    pub mapping_key: Type,
    pub mapping_value: Type,
}

pub fn eval_delegate_entries_via_for<Entry>(
    entry: &Entry,
    table_type: &Type,
) -> syn::Result<Vec<EvaluatedDelegateEntry>>
where
    Entry: EvalForEntries,
{
    let mut entries = Vec::new();

    let for_entries = entry.eval_for_entries(table_type)?;
    for for_entry in for_entries {
        entries.push(for_entry.eval_entry(table_type)?);
    }

    Ok(entries)
}

impl EvalDelegateEntry for EvaluatedForEntry {
    fn eval_entry(&self, _table_type: &Type) -> syn::Result<EvaluatedDelegateEntry> {
        let for_key = &self.for_key;
        let for_value = &self.for_value;
        let mapping_value = &self.mapping_value;
        let table_type = &self.table_type;

        let namespace_trait: Type = {
            // The namespace argument list is extended with the table type and a
            // `Delegate = ..` associated binding. The binding cannot live inside
            // a `TypeArgs` (which faithfully rejects associated bindings), so the
            // trait bound is reconstructed directly from the parsed path and its
            // existing arguments.
            let namespace_path = &self.namespace.path;

            let existing_args = self.namespace.type_args.args.iter();

            parse_internal! {
                #namespace_path < #( #existing_args, )* #table_type, Delegate = #mapping_value >
            }
        };

        let mut generics = self.generics.clone();
        generics.params.push(parse_internal!(#for_key));
        generics.params.push(parse_internal!(#for_value));

        let where_clause = generics.make_where_clause();
        where_clause.predicates.push(parse_internal! {
            #for_key: #namespace_trait
        });

        let entry = EvaluatedDelegateEntry {
            table_type: table_type.clone(),
            generics,
            key: self.mapping_key.clone(),
            value: self.mapping_value.clone(),
        };

        Ok(entry)
    }
}
