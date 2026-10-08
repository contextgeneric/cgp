use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, Error};

/// Error with a spanned "unsupported attribute" message if any attribute is
/// present, pointing at the first one.
pub fn reject_non_empty_attributes(attributes: &[Attribute]) -> syn::Result<()> {
    if let Some(attribute) = attributes.first() {
        Err(Error::new(
            attribute.span(),
            format!(
                "unsupported attribute: {}",
                attribute.path().to_token_stream()
            ),
        ))
    } else {
        Ok(())
    }
}
