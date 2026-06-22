use syn::parse::{Parse, ParseStream};

use crate::types::delegate_and_check_components::CheckParamsAttribute;
use crate::types::delegate_component::DelegateKey;

pub struct SingleDelegateAndCheckKey {
    pub check_params: CheckParamsAttribute,
    pub key: DelegateKey,
}

impl Parse for SingleDelegateAndCheckKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let check_params = input.parse()?;
        let key = input.parse()?;

        Ok(Self { check_params, key })
    }
}
