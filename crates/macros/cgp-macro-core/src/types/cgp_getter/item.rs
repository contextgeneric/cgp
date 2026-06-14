use syn::{Error, TraitItemType};

use crate::functions::parse_getter_fields;
use crate::types::cgp_component::EvaluatedCgpComponent;
use crate::types::cgp_getter::GetterField;

pub struct ItemCgpGetter {
    pub item_component: EvaluatedCgpComponent,
    pub fields: Vec<GetterField>,
    pub field_assoc_type: Option<TraitItemType>,
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
