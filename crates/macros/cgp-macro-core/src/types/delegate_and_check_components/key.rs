use syn::punctuated::Punctuated;

use crate::types::check_components::{
    CheckEntries, CheckEntry, CheckKey, CheckValue, TypeWithGenerics,
};
use crate::types::delegate_and_check_components::{CheckParamsAttribute, ToCheckEntries};
use crate::types::delegate_component::SingleDelegateKey;

impl ToCheckEntries for SingleDelegateKey {
    fn to_check_entries(&self) -> syn::Result<CheckEntries> {
        let check_params = CheckParamsAttribute::parse_attributes(&self.attributes)?;

        match check_params {
            CheckParamsAttribute::None => {
                let entry = CheckEntry {
                    key: CheckKey::Single(self.ty.clone()),
                    value: None,
                };

                Ok(CheckEntries {
                    entries: Punctuated::from_iter([entry]),
                })
            }
            CheckParamsAttribute::Skip => Ok(CheckEntries::default()),
            CheckParamsAttribute::Multi(params) => {
                let mut entries = CheckEntries::default();

                for param in params {
                    entries.entries.push(CheckEntry {
                        key: CheckKey::Single(self.ty.clone()),
                        value: Some(CheckValue::Single(Box::new(TypeWithGenerics::from(param)))),
                    })
                }

                Ok(entries)
            }
        }
    }
}
