use syn::ItemTrait;

use crate::cgp_fn::expand_use_type_attributes_on_trait;
use crate::derive_component::attributes::ComponentAttributes;

pub fn preprocess_consumer_trait(
    consumer_trait: &mut ItemTrait,
    attributes: &ComponentAttributes,
) -> syn::Result<()> {
    consumer_trait.supertraits.extend(attributes.extend.clone());

    if !attributes.use_type.is_empty() {
        *consumer_trait =
            expand_use_type_attributes_on_trait(consumer_trait, &attributes.use_type)?;
    }

    Ok(())
}
