use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::Ident;

use crate::delegate_components::ast::DelegateComponentsAst;
use crate::delegate_components::define_struct::define_struct;
use crate::delegate_components::impl_delegate::impl_delegate_components;
use crate::delegate_components::macro_gen::generate_with_components_macro;
use crate::derive_component::snake_case::to_snake_case_str;

pub fn define_components(body: TokenStream) -> TokenStream {
    let ast: DelegateComponentsAst = syn::parse2(body).unwrap();

    let impl_items = impl_delegate_components(&ast);

    let item_struct = define_struct(&ast.target_ident, &ast.target_generics);

    let mut output = TokenStream::new();

    output.extend(item_struct.to_token_stream());

    for impl_item in impl_items {
        output.extend(impl_item.to_token_stream());
    }

    let components_name = format!("with_{}", to_snake_case_str(&ast.target_ident.to_string()));

    let with_components_macro = generate_with_components_macro(
        &Ident::new(&to_snake_case_str(&components_name), Span::call_site()),
        &ast.all_components(),
    );

    output.extend(with_components_macro);

    output
}
