use syn::ItemTrait;

use crate::derive_component::attributes::CgpComponentAttributes;

pub fn preprocess_consumer_trait(
    consumer_trait: &mut ItemTrait,
    attributes: &CgpComponentAttributes,
) -> syn::Result<()> {
    consumer_trait.supertraits.extend(attributes.extend.clone());

    attributes.use_type.transform_item_trait(consumer_trait)?;

    Ok(())
}
