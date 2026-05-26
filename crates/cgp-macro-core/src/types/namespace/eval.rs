use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemImpl, ItemStruct, ItemTrait};

pub struct EvaluatedNamespaceTable {
    pub item_impls: Vec<ItemImpl>,
    pub item_trait: Option<ItemTrait>,
    pub item_struct: Option<ItemStruct>,
}

impl ToTokens for EvaluatedNamespaceTable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(item_struct) = &self.item_struct {
            item_struct.to_tokens(tokens);
        }

        if let Some(item_trait) = &self.item_trait {
            item_trait.to_tokens(tokens);
        }

        for item_impl in &self.item_impls {
            item_impl.to_tokens(tokens);
        }
    }
}
