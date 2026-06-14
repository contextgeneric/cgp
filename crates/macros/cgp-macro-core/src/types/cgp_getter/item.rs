use syn::TraitItemType;

use crate::types::cgp_component::EvaluatedCgpComponent;
use crate::types::cgp_getter::GetterField;

pub struct ItemCgpGetter {
    pub item_component: EvaluatedCgpComponent,
    pub fields: Vec<GetterField>,
    pub field_assoc_type: Option<TraitItemType>,
}
