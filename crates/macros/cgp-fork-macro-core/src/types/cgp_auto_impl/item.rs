use proc_macro2::Span;
use syn::{Ident, Item, ItemTrait, Type};

use crate::functions::parse_internal;
use crate::types::attributes::CgpComponentAttributes;
use crate::types::cgp_auto_impl::derive_blanket_items;

/// The `#[cgp_auto_impl]` pipeline: a trait whose method bodies are the blanket
/// provider. `preprocess` strips companion attributes; `to_items` emits the trait
/// (bodies removed) and the blanket impl.
pub struct ItemCgpAutoImpl {
    pub item_trait: ItemTrait,
}

impl ItemCgpAutoImpl {
    pub fn preprocess(item_trait: &ItemTrait) -> syn::Result<Self> {
        let (_attributes, item_trait) = CgpComponentAttributes::preprocess(item_trait)?;
        Ok(Self { item_trait })
    }

    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let (item_trait, item_impl) = self.to_blanket_impl()?;

        Ok(vec![item_trait.into(), item_impl.into()])
    }

    /// Build the blanket impl for a fresh `__Context__` generic, inserted ahead
    /// of the trait's own parameters.
    pub fn to_blanket_impl(&self) -> syn::Result<(ItemTrait, syn::ItemImpl)> {
        let context_ident = Ident::new("__Context__", Span::call_site());

        let mut generics = self.item_trait.generics.clone();

        // Insert the context as the leading impl generic. Position 0 is safe with a
        // lifetime present because `syn::Generics::to_tokens` emits lifetimes first.
        // See cgp-knowledge-base/cgp/implementation/README.md, "Generic-parameter insertion and
        // lifetime ordering".
        generics.params.insert(0, parse_internal!(#context_ident));

        let context_type: Type = parse_internal!(#context_ident);

        derive_blanket_items(&context_type, &self.item_trait, generics)
    }
}
