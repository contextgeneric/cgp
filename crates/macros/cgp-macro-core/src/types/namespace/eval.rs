use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemImpl, ItemStruct, ItemTrait};

use crate::types::empty_struct::EmptyStruct;

/// The evaluated `cgp_namespace!` table.
pub struct EvaluatedNamespaceTable {
    /// The generated impls.
    pub item_impls: Vec<ItemImpl>,
    /// The namespace trait, when declared.
    pub item_trait: Option<ItemTrait>,
    /// The namespace struct, when declared.
    pub item_struct: Option<ItemStruct>,
    /// The structs declared for nested `Wrapper<new Inner { … }>` values lifted out
    /// of the body, mirroring `EvaluatedDelegateTable::item_structs`. Their
    /// `DelegateComponent` impls are appended to `item_impls`.
    pub inner_structs: Vec<EmptyStruct>,
}

impl ToTokens for EvaluatedNamespaceTable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(item_struct) = &self.item_struct {
            item_struct.to_tokens(tokens);
        }

        if let Some(item_trait) = &self.item_trait {
            item_trait.to_tokens(tokens);
        }

        for inner_struct in &self.inner_structs {
            inner_struct.to_tokens(tokens);
        }

        for item_impl in &self.item_impls {
            item_impl.to_tokens(tokens);
        }
    }
}
