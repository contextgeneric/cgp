use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ItemTrait;

use crate::derive_component::component_spec::ComponentSpec;
use crate::derive_component::derive::derive_component_with_ast;

pub fn cgp_component(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let spec: ComponentSpec = syn::parse2(attr)?;
    let consumer_trait: ItemTrait = syn::parse2(item)?;

    let derived = derive_component_with_ast(&spec, consumer_trait)?;

    Ok(derived.to_token_stream())
}
