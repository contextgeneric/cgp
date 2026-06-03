use syn::ItemTrait;

use crate::derive_component::attributes::ComponentAttributes;

pub fn preprocess_consumer_trait(
    consumer_trait: &mut ItemTrait,
    attributes: &ComponentAttributes,
) -> syn::Result<()> {
    consumer_trait.supertraits.extend(attributes.extend.clone());

    attributes.use_type.transform_item_trait(consumer_trait)?;

    Ok(())
}
