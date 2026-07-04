use crate::types::delegate_and_check_components::{CheckParamsAttribute, KeyWithCheckParams};
use crate::types::delegate_component::{
    DelegateEntries, DelegateKey, DelegateMapping, MultiDelegateKey, SingleDelegateKey,
    ValidateAttributes,
};

pub trait ToKeysWithCheckParams {
    fn to_keys_with_check_params(&self) -> syn::Result<Vec<KeyWithCheckParams>>;
}

impl ToKeysWithCheckParams for SingleDelegateKey {
    fn to_keys_with_check_params(&self) -> syn::Result<Vec<KeyWithCheckParams>> {
        let check_params = CheckParamsAttribute::parse_attributes(&self.attributes)?;

        // Carry the key's own generics (`<I>` in `<I> FooKey<I>`) into the check
        // entry so the derived check impl binds them; `KeyWithCheckParams` threads
        // them onto each check value.
        let key = KeyWithCheckParams {
            check_params,
            generics: self.generics.clone(),
            key_type: self.ty.clone(),
        };

        Ok(vec![key])
    }
}

impl ToKeysWithCheckParams for MultiDelegateKey {
    fn to_keys_with_check_params(&self) -> syn::Result<Vec<KeyWithCheckParams>> {
        let check_params = CheckParamsAttribute::parse_attributes(&self.attributes)?;

        let mut out = Vec::new();

        for key in &self.keys {
            let inner_res = key.to_keys_with_check_params()?;
            for inner in inner_res {
                let inner_params = check_params.merge(&inner.check_params)?;
                out.push(KeyWithCheckParams {
                    key_type: inner.key_type,
                    generics: inner.generics,
                    check_params: inner_params,
                })
            }
        }

        Ok(out)
    }
}

impl ToKeysWithCheckParams for DelegateKey {
    fn to_keys_with_check_params(&self) -> syn::Result<Vec<KeyWithCheckParams>> {
        match self {
            DelegateKey::Single(key) => key.to_keys_with_check_params(),
            DelegateKey::Multi(key) => key.to_keys_with_check_params(),
            DelegateKey::Path(key) => {
                key.validate_attributes()?;
                Ok(Vec::new())
            }
        }
    }
}

impl ToKeysWithCheckParams for DelegateMapping {
    fn to_keys_with_check_params(&self) -> syn::Result<Vec<KeyWithCheckParams>> {
        match self {
            DelegateMapping::Normal(mapping) => mapping.key.to_keys_with_check_params(),
            DelegateMapping::Direct(mapping) => mapping.key.to_keys_with_check_params(),
            DelegateMapping::Redirect(mapping) => {
                // Redirect mappings do not support check params yet, so reject any
                // attribute on the key rather than silently ignoring it.
                mapping.key.validate_attributes()?;
                Ok(Vec::new())
            }
        }
    }
}

impl ToKeysWithCheckParams for DelegateEntries {
    fn to_keys_with_check_params(&self) -> syn::Result<Vec<KeyWithCheckParams>> {
        let mut out = Vec::new();

        // Statement forms (`for`/`namespace`/`open`) are intentionally not checked
        // for now. They still produce delegate impls via `eval`, but no check
        // entries are generated for them. Since they cannot carry check params,
        // reject any attribute on their keys rather than silently ignoring it.
        for statement in &self.statements {
            statement.validate_attributes()?;
        }

        for entry in &self.entries {
            out.extend(entry.to_keys_with_check_params()?);
        }

        Ok(out)
    }
}
