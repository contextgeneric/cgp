use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, Error};

use crate::types::delegate_component::{MultiDelegateKey, PathDelegateKey, SingleDelegateKey};

/**
    Validate that the attributes in the delegate table constructs are valid.

    At the moment, no attribute is supported, so all attributes are rejected.
*/
pub trait ValidateAttributes {
    fn validate_attributes(&self) -> syn::Result<()>;
}

pub fn reject_non_empty_attributes(attributes: &[Attribute]) -> syn::Result<()> {
    if !attributes.is_empty() {
        let attribute = &attributes[0];
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

impl ValidateAttributes for SingleDelegateKey {
    fn validate_attributes(&self) -> syn::Result<()> {
        reject_non_empty_attributes(&self.attributes)
    }
}

impl ValidateAttributes for MultiDelegateKey {
    fn validate_attributes(&self) -> syn::Result<()> {
        reject_non_empty_attributes(&self.attributes)?;

        for key in &self.keys {
            key.validate_attributes()?;
        }

        Ok(())
    }
}

impl ValidateAttributes for PathDelegateKey {
    fn validate_attributes(&self) -> syn::Result<()> {
        reject_non_empty_attributes(&self.attributes)?;

        Ok(())
    }
}
