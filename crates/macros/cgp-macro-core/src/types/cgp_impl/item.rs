use quote::ToTokens;
use syn::{ItemImpl, Type, parse_quote, parse2};

use crate::traits::AddTypeParamBounds;
use crate::types::attributes::ImplAttributes;
use crate::types::cgp_impl::{ImplArgs, LoweredCgpImpl};
use crate::types::implicits::ImplicitArgFields;

pub struct ItemCgpImpl {
    pub args: ImplArgs,
    pub item_impl: ItemImpl,
}

impl ItemCgpImpl {
    pub fn lower(&self) -> syn::Result<LoweredCgpImpl> {
        let mut item_impl = self.item_impl.clone();

        let attributes = ImplAttributes::parse(&item_impl.attrs)?;
        item_impl.attrs = attributes.raw_attributes;

        let self_type: Type = parse_quote!(Self);

        let implicit_args = ImplicitArgFields::extract_from_impl_items(&mut item_impl.items)?;
        implicit_args.add_type_param_bounds(&self_type, &mut item_impl.generics)?;
        attributes
            .uses
            .add_type_param_bounds(&self_type, &mut item_impl.generics)?;

        attributes.use_type.transform_item_impl(&mut item_impl)?;
        attributes
            .use_provider
            .add_type_param_bounds(&self_type, &mut item_impl.generics)?;

        let (consumer_trait_path, context_type) = match &item_impl.trait_ {
            Some((_, path, _)) => {
                let consumer_trait_path = parse2(path.to_token_stream())?;
                let context_type = item_impl.self_ty.as_ref().clone();
                (consumer_trait_path, context_type)
            }
            None => {
                let consumer_trait_path = parse2(item_impl.self_ty.to_token_stream())?;
                let context_type = parse_quote! { __Context__ };

                item_impl
                    .generics
                    .params
                    .insert(0, parse_quote! { #context_type });

                (consumer_trait_path, context_type)
            }
        };

        Ok(LoweredCgpImpl {
            args: self.args.clone(),
            item_impl,
            context_type,
            consumer_trait_path,
        })
    }
}
