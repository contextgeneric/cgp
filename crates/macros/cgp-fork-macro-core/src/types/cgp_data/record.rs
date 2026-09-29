use syn::parse::{Parse, ParseStream};
use syn::{Ident, Item, ItemImpl, ItemStruct};

use crate::types::cgp_data::{
    derive_builder_struct, derive_finalize_build_impl, derive_has_builder_impl,
    derive_has_field_impls, derive_has_field_impls_from_struct,
    derive_has_fields_impls_from_struct, derive_into_builder_impl,
    derive_partial_data_impl_from_struct, derive_update_field_impls,
};

pub struct ItemCgpRecord {
    pub item_struct: ItemStruct,
}

impl ItemCgpRecord {
    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let has_field_impls = self.to_has_field_impls()?;
        let has_fields_impls = self.to_has_fields_impls()?;
        let build_field_impls = self.to_build_field_items()?;

        let mut items = Vec::new();

        items.extend(has_field_impls.into_iter().map(Item::from));
        items.extend(has_fields_impls.into_iter().map(Item::from));
        items.extend(build_field_impls);

        Ok(items)
    }

    pub fn to_has_field_impls(&self) -> syn::Result<Vec<ItemImpl>> {
        derive_has_field_impls_from_struct(&self.item_struct)
    }

    pub fn to_has_fields_impls(&self) -> syn::Result<Vec<ItemImpl>> {
        derive_has_fields_impls_from_struct(&self.item_struct)
    }

    pub fn to_build_field_items(&self) -> syn::Result<Vec<Item>> {
        let item_struct = &self.item_struct;

        let context_ident = &item_struct.ident;
        let builder_ident = Ident::new(&format!("__Partial{context_ident}"), context_ident.span());

        let builder_struct = derive_builder_struct(item_struct, &builder_ident)?;

        let has_builder_impl = derive_has_builder_impl(item_struct, &builder_ident)?;

        let into_builder_impl = derive_into_builder_impl(item_struct, &builder_ident)?;

        let partial_data_impl = derive_partial_data_impl_from_struct(item_struct, &builder_ident)?;

        let finalize_build_impl = derive_finalize_build_impl(item_struct, &builder_ident)?;

        let update_field_impls = derive_update_field_impls(item_struct, &builder_ident)?;

        let has_field_impls = derive_has_field_impls(item_struct, &builder_ident)?;

        let mut items = vec![
            builder_struct.into(),
            has_builder_impl.into(),
            into_builder_impl.into(),
            partial_data_impl.into(),
            finalize_build_impl.into(),
        ];

        items.extend(update_field_impls.into_iter().map(Item::from));
        items.extend(has_field_impls.into_iter().map(Item::from));

        Ok(items)
    }
}

impl Parse for ItemCgpRecord {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item_struct = input.parse()?;

        Ok(Self { item_struct })
    }
}
