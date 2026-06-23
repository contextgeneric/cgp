use syn::Type;
use syn::punctuated::Punctuated;

use crate::types::check_components::{
    CheckEntries, CheckEntry, CheckKey, CheckValue, TypeWithGenerics,
};
use crate::types::delegate_and_check_components::CheckParamsAttribute;

pub struct KeyWithCheckParams {
    pub key_type: Type,
    pub check_params: CheckParamsAttribute,
}

impl KeyWithCheckParams {
    pub fn to_check_entries(&self) -> CheckEntries {
        match &self.check_params {
            CheckParamsAttribute::Default => {
                let entry = CheckEntry {
                    key: CheckKey::Single(self.key_type.clone()),
                    value: None,
                };

                CheckEntries {
                    entries: Punctuated::from_iter([entry]),
                }
            }
            CheckParamsAttribute::Skip => CheckEntries::default(),
            CheckParamsAttribute::Multi(params) => {
                let mut entries = CheckEntries::default();

                for param in params {
                    entries.entries.push(CheckEntry {
                        key: CheckKey::Single(self.key_type.clone()),
                        value: Some(CheckValue::Single(Box::new(TypeWithGenerics::from(
                            param.clone(),
                        )))),
                    })
                }

                entries
            }
        }
    }
}
