use syn::parse::{Parse, ParseStream};
use syn::{Ident, Item, ItemEnum, ItemImpl};

use crate::types::cgp_data::{
    derive_extract_field_impls, derive_extractor_enum, derive_extractor_enum_ref,
    derive_finalize_extract_impl, derive_from_variant_from_enum, derive_has_extractor_impl,
    derive_has_extractor_mut_impl, derive_has_extractor_ref_impl,
    derive_has_fields_impls_from_enum, derive_partial_data_impl_from_enum,
};

pub struct ItemCgpVariant {
    pub item_enum: ItemEnum,
}

impl ItemCgpVariant {
    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let has_fields = self.to_has_fields_impls()?;
        let from_variant_impls = self.to_from_variant_impls()?;
        let extract_field = self.to_extract_field_items()?;

        let mut items = Vec::new();

        items.extend(has_fields.into_iter().map(Item::from));
        items.extend(from_variant_impls.into_iter().map(Item::from));
        items.extend(extract_field);

        Ok(items)
    }

    pub fn to_from_variant_impls(&self) -> syn::Result<Vec<ItemImpl>> {
        derive_from_variant_from_enum(&self.item_enum)
    }

    pub fn to_has_fields_impls(&self) -> syn::Result<Vec<ItemImpl>> {
        derive_has_fields_impls_from_enum(&self.item_enum)
    }

    pub fn to_extract_field_items(&self) -> syn::Result<Vec<Item>> {
        let item_enum = &self.item_enum;

        let context_ident = &item_enum.ident;

        let extractor_ident =
            Ident::new(&format!("__Partial{context_ident}"), context_ident.span());

        let extractor_enum = derive_extractor_enum(item_enum, &extractor_ident)?;

        let extractor_ref_ident = Ident::new(
            &format!("__PartialRef{context_ident}"),
            context_ident.span(),
        );
        let extractor_ref_enum = derive_extractor_enum_ref(item_enum, &extractor_ref_ident)?;

        let has_extractor_impl = derive_has_extractor_impl(item_enum, &extractor_ident)?;

        let has_extractor_ref_impl =
            derive_has_extractor_ref_impl(item_enum, &extractor_ref_ident)?;

        let has_extractor_mut_impl =
            derive_has_extractor_mut_impl(item_enum, &extractor_ref_ident)?;

        let finalize_extract_impl =
            derive_finalize_extract_impl(item_enum, &extractor_ident, false)?;

        let finalize_extract_ref_impl =
            derive_finalize_extract_impl(item_enum, &extractor_ref_ident, true)?;

        let partial_data_impl =
            derive_partial_data_impl_from_enum(item_enum, &extractor_ident, false)?;

        let partial_ref_data_impl =
            derive_partial_data_impl_from_enum(item_enum, &extractor_ref_ident, true)?;

        let extractor_impls = derive_extract_field_impls(item_enum, &extractor_ident, false)?;

        let extractor_ref_impls =
            derive_extract_field_impls(item_enum, &extractor_ref_ident, true)?;

        let mut items = vec![
            extractor_enum.into(),
            extractor_ref_enum.into(),
            partial_data_impl.into(),
            partial_ref_data_impl.into(),
            has_extractor_impl.into(),
            has_extractor_ref_impl.into(),
            has_extractor_mut_impl.into(),
            finalize_extract_impl.into(),
            finalize_extract_ref_impl.into(),
        ];

        items.extend(extractor_impls.into_iter().map(Item::from));
        items.extend(extractor_ref_impls.into_iter().map(Item::from));

        Ok(items)
    }
}

impl Parse for ItemCgpVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item_enum = input.parse()?;

        Ok(Self { item_enum })
    }
}
