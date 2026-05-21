use syn::token::RArrow;

use crate::types::delegate_component::{DelegateKey, DelegateValue};

pub struct DirectDelegateEntry {
    pub key: DelegateKey,
    pub arrow: RArrow,
    pub value: DelegateValue,
}
