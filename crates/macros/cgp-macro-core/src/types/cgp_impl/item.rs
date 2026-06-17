use quote::ToTokens;
use syn::{ItemImpl, Type};

use crate::functions::parse_internal;
use crate::traits::AddTypeParamBounds;
use crate::types::attributes::CgpImplAttributes;
use crate::types::cgp_impl::{ImplArgs, LoweredCgpImpl};
use crate::types::implicits::ImplicitArgFields;

pub struct ItemCgpImpl {
    pub args: ImplArgs,
    pub item_impl: ItemImpl,
}

impl ItemCgpImpl {
    pub fn lower(&self) -> syn::Result<LoweredCgpImpl> {
        let mut item_impl = self.item_impl.clone();

        let attributes = CgpImplAttributes::parse(&item_impl.attrs)?;
        item_impl.attrs = attributes.raw_attributes;

        let self_type: Type = parse_internal!(Self);

        let implicit_args = ImplicitArgFields::extract_from_impl_items(&mut item_impl.items)?;
        implicit_args.add_type_param_bounds(&self_type, &mut item_impl.generics)?;
        attributes
            .uses
            .add_type_param_bounds(&self_type, &mut item_impl.generics)?;

        attributes.use_type.transform_item_impl(&mut item_impl)?;
        attributes
            .use_provider
            .add_type_param_bounds(&self_type, &mut item_impl.generics)?;

        let default_impls = attributes
            .default_impls
            .to_item_impls(&item_impl.generics, &self.args.provider_type)?;

        let (provider_trait_path, context_type) = match &item_impl.trait_ {
            Some((_, path, _)) => {
                let provider_trait_path = parse_internal(path.to_token_stream())?;
                let context_type = item_impl.self_ty.as_ref().clone();
                (provider_trait_path, context_type)
            }
            None => {
                let provider_trait_path = parse_internal(item_impl.self_ty.to_token_stream())?;
                let context_type = parse_internal! { __Context__ };

                item_impl
                    .generics
                    .params
                    .insert(0, parse_internal! { #context_type });

                (provider_trait_path, context_type)
            }
        };

        Ok(LoweredCgpImpl {
            args: self.args.clone(),
            item_impl,
            context_type,
            provider_trait_path,
            default_impls,
        })
    }
}
