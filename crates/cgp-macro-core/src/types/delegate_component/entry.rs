use syn::parse::{Parse, ParseStream};
use syn::token::{Colon, RArrow};

use crate::types::delegate_component::{DelegateKey, DelegateValue};

pub enum DelegateEntry {
    Normal(NormalDelegateEntry),
    Direct(DirectDelegateEntry),
}

pub struct NormalDelegateEntry {
    pub key: DelegateKey,
    pub colon: Colon,
    pub value: DelegateValue,
}

pub struct DirectDelegateEntry {
    pub key: DelegateKey,
    pub arrow: RArrow,
    pub value: DelegateValue,
}

pub enum DelegateMode {
    Normal(Colon),
    Direct(RArrow),
}

impl Parse for DelegateEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        let mode: DelegateMode = input.parse()?;
        let value = input.parse()?;

        let entry = match mode {
            DelegateMode::Normal(colon) => Self::Normal(NormalDelegateEntry { key, colon, value }),
            DelegateMode::Direct(arrow) => Self::Direct(DirectDelegateEntry { key, arrow, value }),
        };

        Ok(entry)
    }
}

impl Parse for DelegateMode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(RArrow) {
            Ok(Self::Direct(input.parse()?))
        } else {
            Ok(Self::Normal(input.parse()?))
        }
    }
}
