use syn::token::FatArrow;
use syn::{Type, parse_quote};

use crate::types::delegate_component::{
    DelegateKey, EvalDelegateEntry, EvalDelegateKey, EvaluatedDelegateEntry,
};
use crate::types::path::UniPath;

pub struct RedirectDelegateMapping {
    pub key: DelegateKey,
    pub arrow: FatArrow,
    pub value: UniPath,
}

impl EvalDelegateEntry for RedirectDelegateMapping {
    fn eval(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        let value_type: Type = match &self.key {
            DelegateKey::Path(_) => {
                let prefix = self.value.clone().to_prefix(parse_quote!(__Wildcard__));
                parse_quote! {
                    RedirectLookup<#prefix>
                }
            }
            _ => {
                let path = &self.value;
                parse_quote!(RedirectLookup<#path>)
            }
        };

        let mut entries = Vec::new();

        for key in self.key.eval()? {
            let entry = EvaluatedDelegateEntry {
                table_type: table_type.clone(),
                generics: key.generics,
                key: key.key,
                value: value_type.clone(),
            };

            entries.push(entry);
        }

        Ok(entries)
    }
}
