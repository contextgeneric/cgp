use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, Error};

use crate::types::delegate_component::{
    DelegateEntries, DelegateKey, DelegateMapping, DelegateStatement, DelegateTable,
    ForDelegateStatement, MultiDelegateKey, PathDelegateKey, SingleDelegateKey,
};

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

impl ValidateAttributes for DelegateKey {
    fn validate_attributes(&self) -> syn::Result<()> {
        match self {
            DelegateKey::Single(key) => key.validate_attributes(),
            DelegateKey::Multi(key) => key.validate_attributes(),
            DelegateKey::Path(key) => key.validate_attributes(),
        }
    }
}

impl ValidateAttributes for DelegateMapping {
    fn validate_attributes(&self) -> syn::Result<()> {
        match self {
            DelegateMapping::Normal(mapping) => mapping.key.validate_attributes(),
            DelegateMapping::Direct(mapping) => mapping.key.validate_attributes(),
            DelegateMapping::Redirect(mapping) => mapping.key.validate_attributes(),
        }
    }
}

impl ValidateAttributes for ForDelegateStatement {
    fn validate_attributes(&self) -> syn::Result<()> {
        for mapping in &self.mappings {
            mapping.key.validate_attributes()?;
        }

        Ok(())
    }
}

impl ValidateAttributes for DelegateStatement {
    fn validate_attributes(&self) -> syn::Result<()> {
        match self {
            // `namespace` and `open` statements carry no keys that can hold attributes.
            DelegateStatement::Namespace(_) | DelegateStatement::Open(_) => Ok(()),
            DelegateStatement::For(statement) => statement.validate_attributes(),
        }
    }
}

impl ValidateAttributes for DelegateEntries {
    fn validate_attributes(&self) -> syn::Result<()> {
        // Keys nested inside statement forms (`for`/`namespace`/`open`) do not
        // support attributes, so reject any rather than silently discarding them.
        for statement in &self.statements {
            statement.validate_attributes()?;
        }

        for entry in &self.entries {
            entry.validate_attributes()?;
        }

        Ok(())
    }
}

impl ValidateAttributes for DelegateTable {
    fn validate_attributes(&self) -> syn::Result<()> {
        reject_non_empty_attributes(&self.attributes)?;
        self.entries.validate_attributes()?;

        Ok(())
    }
}
