use syn::{Error, Item, TraitItemType};

use crate::functions::parse_getter_fields;
use crate::types::cgp_component::EvaluatedCgpComponent;
use crate::types::cgp_getter::GetterField;
use crate::types::provider_impl::ItemProviderImpls;

/// The evaluated `#[cgp_getter]` component.
pub struct ItemCgpGetter {
    /// The component's items.
    pub item_component: EvaluatedCgpComponent,
    /// The getter's fields.
    pub fields: Vec<GetterField>,
    /// The getter's associated type, when it declares one.
    pub field_assoc_type: Option<TraitItemType>,
}

impl ItemCgpGetter {
    /// Emits the component's items.
    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let mut items = self.item_component.to_items()?;

        let item_impls = self.to_item_provider_impls()?.to_item_impls()?;
        items.extend(item_impls.into_iter().map(Item::Impl));

        Ok(items)
    }

    /// Emits the `UseFields`, `UseField`, and `WithProvider` impls.
    pub fn to_item_provider_impls(&self) -> syn::Result<ItemProviderImpls> {
        let mut items = ItemProviderImpls::default();

        items.items.push(self.to_use_fields_impl()?);
        items.items.extend(self.to_use_field_impl()?);
        items.items.extend(self.to_with_provider_impl()?);

        Ok(items)
    }
}

impl TryFrom<EvaluatedCgpComponent> for ItemCgpGetter {
    type Error = Error;

    fn try_from(item_component: EvaluatedCgpComponent) -> Result<Self, Self::Error> {
        let (fields, field_assoc_type) = parse_getter_fields(
            &item_component.args.context_ident,
            &item_component.consumer_trait,
        )?;
        Ok(Self {
            item_component,
            fields,
            field_assoc_type,
        })
    }
}
