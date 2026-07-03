use syn::Type;
use syn::token::FatArrow;

use crate::exports::RedirectLookup;
use crate::parse_internal;
use crate::types::delegate_component::{
    DelegateKey, EvalDelegateEntries, EvalDelegateKey, EvaluatedDelegateEntry,
};
use crate::types::path::UniPath;

/// A `Key => @path` mapping that redirects the lookup along an `@`-path:
/// `Delegate` becomes `RedirectLookup<TableType, Path>`.
#[derive(Debug, Clone)]
pub struct RedirectDelegateMapping {
    pub key: DelegateKey,
    pub arrow: FatArrow,
    pub value: UniPath,
}

impl EvalDelegateEntries for RedirectDelegateMapping {
    fn eval_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        let value_type: Type = match &self.key {
            DelegateKey::Path(_) => {
                let prefix = self.value.clone().to_prefix(parse_internal!(__Wildcard__));
                parse_internal! {
                    #RedirectLookup<#table_type, #prefix>
                }
            }
            _ => {
                let path = &self.value;
                parse_internal!(#RedirectLookup<#table_type, #path>)
            }
        };

        let mut entries = Vec::new();

        for key in self.key.eval()? {
            let entry = EvaluatedDelegateEntry {
                table_type: table_type.clone(),
                generics: key.generics,
                key: key.key,
                value: value_type.clone(),
                span: key.span,
            };

            entries.push(entry);
        }

        Ok(entries)
    }
}
