use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Error, Ident, ItemImpl, ItemTrait, parse2};

use crate::macro_core::types::cgp_component::{CgpComponentArgs, ItemCgpComponent};
use crate::macro_core::types::cgp_getter::ItemCgpGetter;
use crate::macro_core::types::cgp_type::{ItemCgpType, extract_item_type_from_trait};

/// `#[derive_provider(WithProvider)]` emits the `WithProvider` impl for a
/// `#[cgp_component]` trait. The `WithProvider` struct already lives in
/// `cgp-fork-component`; this macro only generates the impl (and its `IsProviderFor`).
pub fn derive_provider(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let kind: Ident = parse2(attr)?;

    if kind != "WithProvider" {
        return Err(Error::new(
            kind.span(),
            "`derive_provider` only supports `WithProvider`",
        ));
    }

    let item_trait: ItemTrait = parse2(item.clone())?;
    let component_attr = item_trait
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("cgp_component"))
        .ok_or_else(|| {
            Error::new(
                item_trait.span(),
                "`#[derive_provider(WithProvider)]` must sit on a `#[cgp_component]` trait",
            )
        })?;

    let mut trait_for_pipeline = item_trait.clone();
    trait_for_pipeline
        .attrs
        .retain(|attr| !attr.path().is_ident("cgp_component"));

    let args: CgpComponentArgs = component_attr.parse_args()?;
    let evaluated = ItemCgpComponent {
        args,
        item_trait: trait_for_pipeline,
    }
    .preprocess()?
    .eval()?;

    let provider_impls: Vec<ItemImpl> =
        if extract_item_type_from_trait(&evaluated.consumer_trait).is_ok() {
            let item = ItemCgpType {
                item_component: evaluated,
            };
            item.to_with_provider_impl()?.to_item_impls()?
        } else {
            let item = ItemCgpGetter::try_from(evaluated)?;
            let provider = item.to_with_provider_impl()?.ok_or_else(|| {
                Error::new(
                    item_trait.span(),
                    "`#[derive_provider(WithProvider)]` supports a getter with a single method",
                )
            })?;
            provider.to_item_impls()?
        };

    // Re-emit the original trait so the inner `#[cgp_component]` still expands.
    // This attribute has to be the outer one.
    Ok(quote! {
        #item
        #(#provider_impls)*
    })
}
