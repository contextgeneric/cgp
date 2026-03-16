use syn::{Attribute, Ident, Meta};

pub fn parse_delegate_attributes(attributes: Vec<Attribute>) -> syn::Result<DelegateAttributes> {
    let mut parsed_attributes = DelegateAttributes::default();

    for attribute in attributes.iter() {
        if let Some(ident) = attribute.path().get_ident() {
            if ident == "use_namespace" {
                if parsed_attributes.use_namespace.is_some() {
                    return Err(syn::Error::new_spanned(
                        attribute,
                        "Multiple #[use_namespace] attributes are not allowed",
                    ));
                }

                if let Meta::Path(_) = attribute.meta {
                    parsed_attributes.use_namespace =
                        Some(DelegateNamespaceAttribute { namespace: None });
                } else {
                    let namespace = attribute.parse_args::<Option<Ident>>()?;
                    parsed_attributes.use_namespace =
                        Some(DelegateNamespaceAttribute { namespace });
                }
            } else {
                return Err(syn::Error::new_spanned(
                    attribute,
                    format!("Unknown attribute {} for delegate_components", ident),
                ));
            }
        } else {
            return Err(syn::Error::new_spanned(
                attribute,
                "Unexpected attribute format for delegate_components",
            ));
        }
    }

    Ok(parsed_attributes)
}

#[derive(Default)]
pub struct DelegateAttributes {
    pub use_namespace: Option<DelegateNamespaceAttribute>,
}

pub struct DelegateNamespaceAttribute {
    pub namespace: Option<Ident>,
}
