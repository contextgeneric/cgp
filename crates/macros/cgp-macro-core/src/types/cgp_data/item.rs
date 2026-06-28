use syn::parse::{Parse, ParseStream};
use syn::{Error, Item};

use crate::types::cgp_data::{ItemCgpRecord, ItemCgpVariant};

pub enum ItemCgpData {
    Record(ItemCgpRecord),
    Variant(ItemCgpVariant),
}

impl ItemCgpData {
    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        match self {
            ItemCgpData::Record(record) => record.to_items(),
            ItemCgpData::Variant(variant) => variant.to_items(),
        }
    }
}

impl Parse for ItemCgpData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item: Item = input.parse()?;

        match item {
            Item::Struct(item_struct) => Ok(Self::Record(ItemCgpRecord { item_struct })),
            Item::Enum(item_enum) => Ok(Self::Variant(ItemCgpVariant { item_enum })),
            _ => Err(Error::new_spanned(
                item,
                "expect body to be either a struct or enum",
            )),
        }
    }
}
