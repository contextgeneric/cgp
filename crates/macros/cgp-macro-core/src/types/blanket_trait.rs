use syn::{Ident, ImplItem, ItemImpl, ItemTrait, TraitItem, TypeParamBound, WherePredicate};

use crate::parse_internal;

pub struct ItemBlanketTrait {
    pub context_ident: Ident,
    pub item_trait: ItemTrait,
}

impl ItemBlanketTrait {
    pub fn to_item_impl(&self) -> syn::Result<ItemImpl> {
        let Self { context_ident, item_trait } = self;
        todo!()
    }
}
