use core::mem;

use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, ItemTrait, TypeParamBound};

use crate::types::attributes::{
    DeriveDelegateAttribute, DeriveDelegateAttributes, PrefixAttribute, UseTypeAttribute,
    UseTypeAttributes,
};

/// The modifier attributes a `#[cgp_component]` trait carries.
#[derive(Default, Clone)]
pub struct CgpComponentAttributes {
    /// The `#[extend]` supertraits.
    pub extend: Vec<TypeParamBound>,
    /// The `#[use_type]` imports.
    pub use_type: UseTypeAttributes,
    /// The `#[prefix]` registrations.
    pub prefixes: Vec<PrefixAttribute>,
    /// The `#[derive_delegate]` tables.
    pub derive_delegate_attributes: DeriveDelegateAttributes,
}

impl CgpComponentAttributes {
    /// Parses the attributes and applies them to the trait.
    pub fn preprocess(item_trait: &ItemTrait) -> syn::Result<(Self, ItemTrait)> {
        let mut item_trait = item_trait.clone();

        let attributes = Self::parse(&mut item_trait.attrs)?;

        item_trait.supertraits.extend(attributes.extend.clone());

        attributes.use_type.transform_item_trait(&mut item_trait)?;

        Ok((attributes, item_trait))
    }

    /// Parses the trait's modifier attributes.
    pub fn parse(attributes: &mut Vec<Attribute>) -> syn::Result<Self> {
        let mut parsed_attributes = CgpComponentAttributes::default();

        let in_attributes = mem::take(attributes);

        for attribute in in_attributes.into_iter() {
            if let Some(ident) = attribute.path().get_ident() {
                if ident == "extend" {
                    let extend_bound = attribute
                        .parse_args_with(Punctuated::<TypeParamBound, Comma>::parse_terminated)?;
                    parsed_attributes.extend.extend(extend_bound);
                } else if ident == "use_type" {
                    let use_type_specs = attribute
                        .parse_args_with(Punctuated::<UseTypeAttribute, Comma>::parse_terminated)?;

                    for use_type_spec in use_type_specs.iter() {
                        for type_ident in use_type_spec.type_idents.iter() {
                            if let Some(equals) = &type_ident.equals {
                                return Err(syn::Error::new_spanned(
                                    equals,
                                    "Type equality constraints cannot be used in component trait definition",
                                ));
                            }
                        }
                    }

                    parsed_attributes.use_type.attributes.extend(use_type_specs);
                } else if ident == "prefix" {
                    let namespace_specs = attribute.parse_args_with(PrefixAttribute::parse)?;
                    parsed_attributes.prefixes.push(namespace_specs);
                } else if ident == "uses" {
                    // `#[uses]` imports a provider's dependencies; on a trait definition it would
                    // only be left for the compiler, which knows no such attribute.
                    let bounds = &attribute.meta.require_list()?.tokens;
                    return Err(syn::Error::new_spanned(
                        &attribute,
                        format!(
                            "`#[uses]` declares the dependencies of a provider, not of a trait; write `#[extend({bounds})]` to make it a supertrait"
                        ),
                    ));
                } else if ident == "derive_delegate" {
                    let derive_delegate_attribute =
                        attribute.parse_args_with(DeriveDelegateAttribute::parse)?;
                    parsed_attributes
                        .derive_delegate_attributes
                        .attributes
                        .push(derive_delegate_attribute);
                } else {
                    attributes.push(attribute);
                }
            } else {
                attributes.push(attribute);
            }
        }

        Ok(parsed_attributes)
    }
}
