use syn::{ImplItem, ItemTrait, Type};

use crate::functions::trait_items_to_delegated_impl_items;
use crate::parse_internal;

pub fn provider_trait_to_impl_items(
    item_trait: &ItemTrait,
    delegate_type: &Type,
) -> syn::Result<Vec<ImplItem>> {
    let provider_name = &item_trait.ident;
    let provider_type_generics = item_trait.generics.split_for_impl().1;
    let provider_trait_path: Type = parse_internal!(#provider_name #provider_type_generics);

    trait_items_to_delegated_impl_items(&item_trait.items, delegate_type, &provider_trait_path)
}
