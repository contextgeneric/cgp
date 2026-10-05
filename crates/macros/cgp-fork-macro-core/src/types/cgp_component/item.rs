use syn::{Error, GenericParam, ItemTrait};

use crate::types::attributes::CgpComponentAttributes;
use crate::types::cgp_component::{CgpComponentArgs, PreprocessedCgpComponent};
use crate::types::ident::TypeGenericParam;

/// Raw input stage: the parsed attribute args and trait, before CGP attributes
/// are stripped. First stage of the `#[cgp_component]` pipeline.
pub struct ItemCgpComponent {
    pub args: CgpComponentArgs,
    pub item_trait: ItemTrait,
}

impl ItemCgpComponent {
    /// Split the CGP modifier attributes off the trait, yielding the next stage.
    pub fn preprocess(&self) -> syn::Result<PreprocessedCgpComponent> {
        self.check_component_name_params()?;

        let (attributes, item_trait) = CgpComponentAttributes::preprocess(&self.item_trait)?;

        Ok(PreprocessedCgpComponent {
            args: self.args.clone(),
            item_trait,
            attributes,
        })
    }

    /// Reject a `name:` parameter the trait does not declare. The marker struct is
    /// declared with the name's parameters and the generated impls name it with the
    /// trait's, so an undeclared one would otherwise fail downstream with `E0425`.
    fn check_component_name_params(&self) -> syn::Result<()> {
        let trait_generics = &self.item_trait.generics;

        for param in self.args.component_name.type_generics.params.iter() {
            let (declared, name) = match param {
                TypeGenericParam::Lifetime(life) => (
                    trait_generics.params.iter().any(
                        |p| matches!(p, GenericParam::Lifetime(l) if l.lifetime.ident == life.ident),
                    ),
                    life.to_string(),
                ),
                TypeGenericParam::Type(ident) => (
                    trait_generics
                        .params
                        .iter()
                        .any(|p| matches!(p, GenericParam::Type(t) if &t.ident == ident)),
                    ident.to_string(),
                ),
                TypeGenericParam::Const(param) => (
                    trait_generics
                        .params
                        .iter()
                        .any(|p| matches!(p, GenericParam::Const(c) if c.ident == param.ident)),
                    param.ident.to_string(),
                ),
            };

            if !declared {
                return Err(Error::new_spanned(
                    param,
                    format!(
                        "the component name's parameter `{name}` is not a generic parameter of the trait `{}`",
                        self.item_trait.ident,
                    ),
                ));
            }
        }

        Ok(())
    }
}
