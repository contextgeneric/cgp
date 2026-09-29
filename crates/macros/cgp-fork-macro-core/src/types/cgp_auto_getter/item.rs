use proc_macro2::Span;
use syn::{Ident, Item, ItemImpl, ItemTrait};

use crate::functions::parse_getter_fields;
use crate::types::attributes::CgpComponentAttributes;
use crate::types::cgp_auto_getter::derive_blanket_impl;

pub struct ItemCgpAutoGetter {
    pub item_trait: ItemTrait,
}

impl ItemCgpAutoGetter {
    pub fn preprocess(item_trait: &ItemTrait) -> syn::Result<Self> {
        let (_attributes, item_trait) = CgpComponentAttributes::preprocess(item_trait)?;
        Ok(Self { item_trait })
    }

    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let item_trait = self.item_trait.clone().into();
        let item_impl = self.to_blanket_impl()?.into();

        Ok(vec![item_trait, item_impl])
    }

    pub fn to_blanket_impl(&self) -> syn::Result<ItemImpl> {
        let context_type = Ident::new("__Context__", Span::call_site());

        let (fields, field_type) = parse_getter_fields(&context_type, &self.item_trait)?;

        let blanket_impl =
            derive_blanket_impl(&context_type, &self.item_trait, &fields, &field_type)?;

        Ok(blanket_impl)
    }
}
