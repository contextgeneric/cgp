use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::delegate_components::ast::DelegateComponentsAst;
use crate::delegate_components::impl_delegate::impl_delegate_components;

pub fn delegate_components(body: TokenStream) -> TokenStream {
    let ast: DelegateComponentsAst = syn::parse2(body).unwrap();

    let impl_items = impl_delegate_components(
        &ast.target_type,
        &ast.target_generics,
        &ast.delegate_entries,
    );

    let mut output = TokenStream::new();

    for impl_item in impl_items {
        output.extend(impl_item.to_token_stream());
    }

    output
}
