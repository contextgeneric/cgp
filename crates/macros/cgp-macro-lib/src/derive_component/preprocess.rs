use cgp_macro_core::types::attributes::CgpComponentAttributes;
use syn::ItemTrait;

pub fn preprocess_consumer_trait(
    consumer_trait: &mut ItemTrait,
    attributes: &CgpComponentAttributes,
) -> syn::Result<()> {
    consumer_trait.supertraits.extend(attributes.extend.clone());

    attributes.use_type.transform_item_trait(consumer_trait)?;

    Ok(())
}
