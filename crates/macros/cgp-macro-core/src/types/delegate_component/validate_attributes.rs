use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, Error};

use crate::types::delegate_component::{
    DelegateEntries, DelegateKey, DelegateMapping, DelegateStatement, DelegateTable, DelegateValue,
    DelegateValueWithInnerTable, ForDelegateStatement, MultiDelegateKey, PathDelegateKey,
    SingleDelegateKey,
};

/**
    Validate that the attributes in the delegate table constructs are valid.

    At the moment, no attribute is supported, so all attributes are rejected.
*/
pub trait ValidateAttributes {
    fn validate_attributes(&self) -> syn::Result<()>;
}

/// Error with a spanned "unsupported attribute" message if any attribute is
/// present, pointing at the first one.
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

impl ValidateAttributes for DelegateValueWithInnerTable {
    fn validate_attributes(&self) -> syn::Result<()> {
        // The wrapper cannot carry attributes, but the inner table's keys can, so
        // recurse into it rather than letting an attribute be silently dropped.
        self.inner_table.entries.validate_attributes()
    }
}

impl ValidateAttributes for DelegateValue {
    fn validate_attributes(&self) -> syn::Result<()> {
        match self {
            DelegateValue::Type(_) => Ok(()),
            DelegateValue::WithTable(value) => value.validate_attributes(),
        }
    }
}

impl ValidateAttributes for DelegateMapping {
    fn validate_attributes(&self) -> syn::Result<()> {
        match self {
            // A Normal or Direct value may open a nested inner table whose keys
            // can hold attributes, so validate the value as well as the key.
            DelegateMapping::Normal(mapping) => {
                mapping.key.validate_attributes()?;
                mapping.value.validate_attributes()
            }
            DelegateMapping::Direct(mapping) => {
                mapping.key.validate_attributes()?;
                mapping.value.validate_attributes()
            }
            // A Redirect value is a bare `@`-path with no inner table.
            DelegateMapping::Redirect(mapping) => mapping.key.validate_attributes(),
        }
    }
}

impl ValidateAttributes for ForDelegateStatement {
    fn validate_attributes(&self) -> syn::Result<()> {
        for mapping in &self.mappings {
            mapping.key.validate_attributes()?;
            mapping.value.validate_attributes()?;
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
