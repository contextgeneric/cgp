use crate::types::delegate_and_check_components::{CheckParamsAttribute, KeyWithCheckParams};
use crate::types::delegate_component::SingleDelegateKey;

pub trait ToKeysWithCheckParams {
    fn to_keys_with_check_params(&self) -> syn::Result<Vec<KeyWithCheckParams>>;
}

impl ToKeysWithCheckParams for SingleDelegateKey {
    fn to_keys_with_check_params(&self) -> syn::Result<Vec<KeyWithCheckParams>> {
        let check_params = CheckParamsAttribute::parse_attributes(&self.attributes)?;

        let key = KeyWithCheckParams {
            check_params,
            key_type: self.ty.clone(),
        };

        Ok(vec![key])
    }
}
