use cgp_macro_core::types::cgp_component::{
    CgpComponentArgs, EvaluatedCgpComponent, ItemCgpComponent,
};
use cgp_macro_core::types::empty_struct::EmptyStruct;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::{ItemImpl, ItemTrait};

pub fn derive_component_with_ast(
    args: &CgpComponentArgs,
    item_trait: ItemTrait,
) -> syn::Result<DerivedComponent> {
    let item = ItemCgpComponent {
        args: args.clone(),
        item_trait,
    };

    let preprocessed = item.preprocess()?;

    let evaluated = preprocessed.eval()?;

    let mut item_impls = evaluated.to_item_impls()?;

    let EvaluatedCgpComponent {
        component_struct,
        consumer_trait,
        consumer_impl,
        provider_trait,
        provider_impl,
        ..
    } = evaluated;

    item_impls.push(consumer_impl);
    item_impls.push(provider_impl);

    let derived = DerivedComponent {
        component_struct,
        consumer_trait,
        provider_trait,
        item_impls,
    };

    Ok(derived)
}

pub struct DerivedComponent {
    pub component_struct: EmptyStruct,
    pub consumer_trait: ItemTrait,
    pub provider_trait: ItemTrait,
    pub item_impls: Vec<ItemImpl>,
}

impl ToTokens for DerivedComponent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.component_struct.to_tokens(tokens);
        self.consumer_trait.to_tokens(tokens);
        self.provider_trait.to_tokens(tokens);
        tokens.append_all(self.item_impls.iter());
    }
}
