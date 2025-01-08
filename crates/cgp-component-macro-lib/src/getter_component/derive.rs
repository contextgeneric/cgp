use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemTrait;

use crate::derive_component::component_spec::ComponentSpec;
use crate::derive_component::derive::derive_component_with_ast;

pub fn derive_getter_component(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let spec: ComponentSpec = syn::parse2(attr).unwrap();
    let consumer_trait: ItemTrait = syn::parse2(item).unwrap();

    let derived_component = derive_component_with_ast(spec, consumer_trait)?;

    Ok(quote! {
        #derived_component
    })
}
