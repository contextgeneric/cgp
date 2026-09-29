use syn::punctuated::Punctuated;
use syn::{Type, parse_quote};

use crate::types::check_components::{
    CheckEntries, CheckEntry, CheckKey, CheckValue, TypeWithGenerics,
};
use crate::types::delegate_and_check_components::CheckParamsAttribute;
use crate::types::generics::ImplGenerics;

/// A delegation key paired with the check control derived from its attributes and
/// its own generic parameters. The generics are threaded onto every check value so
/// a key that introduces its own parameters (`<I> FooKey<I>`) binds them on the
/// generated check impl rather than referencing them unbound.
pub struct KeyWithCheckParams {
    pub key_type: Type,
    pub generics: ImplGenerics,
    pub check_params: CheckParamsAttribute,
}

impl KeyWithCheckParams {
    pub fn to_check_entries(&self) -> CheckEntries {
        match &self.check_params {
            CheckParamsAttribute::Default => {
                // The default check has unit params. When the key carries its own
                // generics, attach them to that unit value so the impl binds them
                // (`impl<I> __Check<FooKey<I>, ()> for Context {}`); a key with no
                // generics keeps the bare `value: None` form, which also lowers to
                // unit params but without an empty generic list.
                let value = if self.generics.params.is_empty() {
                    None
                } else {
                    Some(CheckValue::Single(Box::new(TypeWithGenerics {
                        ty: parse_quote!(()),
                        generics: self.generics.clone(),
                    })))
                };

                let entry = CheckEntry {
                    key: CheckKey::Single(self.key_type.clone()),
                    value,
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
                        value: Some(CheckValue::Single(Box::new(TypeWithGenerics {
                            ty: param.clone(),
                            generics: self.generics.clone(),
                        }))),
                    })
                }

                entries
            }
        }
    }
}
