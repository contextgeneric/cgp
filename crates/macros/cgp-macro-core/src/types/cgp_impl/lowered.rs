use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::token::For;
use syn::visit_mut::VisitMut;
use syn::{Error, Ident, ImplItem, ItemImpl, Type};

use crate::functions::{parse_internal, to_snake_case_ident};
use crate::types::cgp_impl::{CgpProviderOrBareImpl, ImplArgs};
use crate::types::cgp_provider::{ItemCgpProvider, ProviderArgs};
use crate::types::ident::PathWithTypeArgs;
use crate::visitors::{
    ReplaceSelfReceiverVisitor, ReplaceSelfTypeVisitor, ReplaceSelfValueVisitor,
};

/// The header-normalized stage of `#[cgp_impl]`, carrying the resolved context
/// type and provider trait path. It owns the consumer-to-provider rewrite.
pub struct LoweredCgpImpl {
    pub args: ImplArgs,
    pub item_impl: ItemImpl,
    pub context_type: Type,
    pub provider_trait_path: PathWithTypeArgs,
    pub default_impls: Vec<ItemImpl>,
}

impl LoweredCgpImpl {
    /// Rewrites the block into provider-trait form and hands it to
    /// [`ItemCgpProvider`], or — for the `#[cgp_impl(Self)]` case — returns the
    /// block unchanged as a bare consumer impl (requiring a `for` clause).
    pub fn lower(&self) -> syn::Result<CgpProviderOrBareImpl> {
        if self.args.provider_type == parse_internal!(Self) {
            if self.item_impl.trait_.is_none() {
                return Err(Error::new(
                    self.item_impl.span(),
                    "Expected context type to be specified",
                ));
            }

            Ok(CgpProviderOrBareImpl::Bare(Box::new(
                self.item_impl.clone(),
            )))
        } else {
            let provider_impl = self.to_raw_item_impl()?;

            let item_cgp_provider = ItemCgpProvider {
                args: ProviderArgs {
                    new: self.args.new.clone(),
                    component_type: self.args.component_type.clone(),
                },
                item_impl: provider_impl,
            };

            let lowered = item_cgp_provider.lower()?;

            Ok(CgpProviderOrBareImpl::Provider(Box::new(lowered)))
        }
    }

    /// Performs the consumer-to-provider rewrite: swaps `Self` to the provider,
    /// inserts the context as the provider trait's leading argument, and runs the
    /// three `replace_self` visitors to rewrite `self`/`Self` in the body.
    pub fn to_raw_item_impl(&self) -> syn::Result<ItemImpl> {
        let item_impl = &self.item_impl;
        let context_type = &self.context_type;
        let mut provider_trait_path = self.provider_trait_path.clone();
        let provider_type = &self.args.provider_type;

        let context_ident =
            if let Ok(ident) = parse_internal::<Ident>(context_type.to_token_stream()) {
                to_snake_case_ident(&ident)
            } else {
                Ident::new("__context__", Span::call_site())
            };

        let local_assoc_types: Vec<Ident> = item_impl
            .items
            .iter()
            .filter_map(|item| {
                if let ImplItem::Type(assoc_type) = item {
                    Some(assoc_type.ident.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut out_impl = item_impl.clone();

        out_impl.self_ty = Box::new(provider_type.clone());

        // Insert the context as the provider trait's leading type argument.
        // Position 0 places it ahead of any lifetime argument, which is invalid
        // Rust ordering on its own — it is only safe because the path is
        // re-parsed through `syn` below (`parse_internal(… .to_token_stream())`),
        // and `syn` re-emits lifetimes first. See cgp-knowledge-base/cgp/implementation/README.md,
        // "Generic-parameter insertion and lifetime ordering".
        provider_trait_path
            .type_args
            .args
            .insert(0, parse_internal!(#context_type));

        // Reuse the user's `for` token span when the block was written in the
        // explicit `impl Trait for Context` form; otherwise (the bare `impl Trait`
        // form) aim the synthesized `for` at the provider trait the user wrote, so
        // no structural token of the generated impl carries the macro `call_site`.
        let for_token = match &item_impl.trait_ {
            Some((_, _, for_token)) => *for_token,
            None => For(self.provider_trait_path.span()),
        };

        out_impl.trait_ = Some((
            None,
            parse_internal(provider_trait_path.to_token_stream())?,
            for_token,
        ));

        ReplaceSelfTypeVisitor {
            replaced_type: context_type,
            skip_assoc_types: &local_assoc_types,
        }
        .visit_item_impl_mut(&mut out_impl);

        ReplaceSelfReceiverVisitor {
            replaced_ident: &context_ident,
            replaced_type: context_type,
        }
        .visit_item_impl_mut(&mut out_impl);

        ReplaceSelfValueVisitor {
            replaced_ident: &context_ident,
        }
        .visit_item_impl_mut(&mut out_impl);

        Ok(out_impl)
    }
}
