use syn::{ItemEnum, ItemImpl};

use crate::types::cgp_data::derive_from_variant_from_enum;

pub struct ItemCgpVariant {
    pub item_enum: ItemEnum,
}

impl ItemCgpVariant {
    pub fn to_from_variant_impls(&self) -> syn::Result<Vec<ItemImpl>> {
        derive_from_variant_from_enum(&self.item_enum)
    }
}
