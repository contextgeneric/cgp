use syn::ItemTrait;

use crate::cgp_fn::expand_use_type_attributes_on_trait;
use crate::derive_component::attributes::parse_component_attributes;

pub fn preprocess_consumer_trait(consumer_trait: &mut ItemTrait) -> syn::Result<()> {
    let attributes = parse_component_attributes(&mut consumer_trait.attrs)?;

    consumer_trait.supertraits.extend(attributes.extend.clone());

    if !attributes.use_type.is_empty() {
        *consumer_trait =
            expand_use_type_attributes_on_trait(consumer_trait, &attributes.use_type)?;
    }

    Ok(())
}
