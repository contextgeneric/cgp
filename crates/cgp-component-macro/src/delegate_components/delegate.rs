use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse_quote;

use crate::delegate_components::ast::DelegateComponentsAst;
use crate::delegate_components::impl_delegate::impl_delegate_components;
use crate::delegate_components::macro_gen::generate_with_components_macro;

pub fn delegate_components(body: TokenStream) -> TokenStream {
    let ast: DelegateComponentsAst = syn::parse2(body).unwrap();

    let impl_items = impl_delegate_components(&ast);

    let with_components_macro =
        generate_with_components_macro(&parse_quote!(with_components), &ast.all_components());

    let mut output = TokenStream::new();

    for impl_item in impl_items {
        output.extend(impl_item.to_token_stream());
    }

    output.extend(with_components_macro);

    output
}
