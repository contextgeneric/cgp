use syn::token::Colon;

use crate::types::delegate_component::{DelegateKey, DelegateValue};

pub struct NormalDelegateEntry {
    pub key: DelegateKey,
    pub colon: Colon,
    pub value: DelegateValue,
}
