use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::token::For;
use syn::visit_mut::VisitMut;
use syn::{Error, Ident, ImplItem, ItemImpl, Path, Type};

use crate::functions::{parse_internal, to_snake_case_ident};
use crate::types::cgp_impl::{CgpProviderOrBareImpl, ImplArgs};
use crate::types::cgp_provider::{ItemCgpProvider, ProviderArgs};
use crate::types::ident::PathWithTypeArgs;
use crate::visitors::{
    QualifySelfConstVisitor, ReplaceSelfReceiverVisitor, ReplaceSelfTypeVisitor,
    ReplaceSelfValueVisitor,
};

/// The header-normalized stage of `#[cgp_impl]`, carrying the resolved context
/// type and provider trait path. It owns the consumer-to-provider rewrite.
pub struct LoweredCgpImpl {
    /// The impl's arguments.
    pub args: ImplArgs,
    /// The impl block.
    pub item_impl: ItemImpl,
    /// The context type.
    pub context_type: Type,
    /// The provider trait's path.
    pub provider_trait_path: PathWithTypeArgs,
    /// The `#[default_impl]` impls.
    pub default_impls: Vec<ItemImpl>,
}

impl LoweredCgpImpl {
    /// Rewrites the block into provider-trait form and hands it to
    /// [`ItemCgpProvider`], or — for the `#[cgp_impl(Self)]` case — returns the
    /// block unchanged as a bare consumer impl (requiring a `for` clause).
    /// Lowers the block into a provider or a bare consumer impl.
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
    /// Rewrites the block into provider-trait form.
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

        // `Self::Item` naming an associated type or const the block declares
        // means the provider's own item, so the rewrite leaves it alone.
        let local_assoc_types: Vec<Ident> = item_impl
            .items
            .iter()
            .filter_map(|item| match item {
                ImplItem::Type(assoc_type) => Some(assoc_type.ident.clone()),
                _ => None,
            })
            .collect();
        let local_assoc_consts: Vec<Ident> = item_impl
            .items
            .iter()
            .filter_map(|item| match item {
                ImplItem::Const(assoc_const) => Some(assoc_const.ident.clone()),
                _ => None,
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

        let provider_trait: Path = parse_internal(provider_trait_path.to_token_stream())?;

        // A local const is named through the provider trait, since a bare
        // `Self::CONST` on the provider is ambiguous with the consumer trait's.
        QualifySelfConstVisitor {
            local_consts: &local_assoc_consts,
            qualified_self: &quote!(<#provider_type as #provider_trait>),
        }
        .visit_item_impl_mut(&mut out_impl);

        out_impl.trait_ = Some((None, provider_trait, for_token));

        ReplaceSelfTypeVisitor {
            replaced_type: context_type,
            skip_assoc_items: &local_assoc_types,
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
