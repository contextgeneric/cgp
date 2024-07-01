use proc_macro2::{Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::Ident;

use crate::delegate_components::ast::DelegateComponentsAst;
use crate::delegate_components::impl_delegate::impl_delegate_components;
use crate::delegate_components::macro_gen::generate_with_components_macro;
use crate::derive_component::snake_case::to_snake_case_str;

pub fn delegate_components(body: TokenStream) -> TokenStream {
    let ast: DelegateComponentsAst = syn::parse2(body).unwrap();

    let impl_items = impl_delegate_components(&ast);

    let mut output = TokenStream::new();

    for impl_item in impl_items {
        output.extend(impl_item.to_token_stream());
    }

    output
}
