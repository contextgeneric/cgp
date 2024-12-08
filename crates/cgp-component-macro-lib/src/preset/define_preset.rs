use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{parse_quote, Ident, ItemTrait};

use crate::delegate_components::define_struct::define_struct;
use crate::delegate_components::delegates_to::define_delegates_to_trait;
use crate::delegate_components::impl_delegate::impl_delegate_components;
use crate::derive_component::snake_case::to_snake_case_str;
use crate::preset::ast::DefinePresetAst;
use crate::preset::impl_is_preset::impl_components_is_preset;
use crate::preset::substitution_macro::define_substitution_macro;

pub fn define_preset(body: TokenStream) -> syn::Result<TokenStream> {
    let ast: DefinePresetAst = syn::parse2(body)?;

    let preset_ident = &ast.preset_ident;

    let preset_type = {
        let type_generics = ast.preset_generics.split_for_impl().1;
        parse_quote!( #preset_ident #type_generics )
    };

    let preset_trait_name = Ident::new(&format!("Is{}", preset_ident), preset_ident.span());

    let preset_trait: ItemTrait = parse_quote! {
        pub trait #preset_trait_name <Component> {}
    };

    let impl_delegate_items =
        impl_delegate_components(&preset_type, &ast.preset_generics, &ast.delegate_entries);

    let impl_is_reset_items = impl_components_is_preset(
        &preset_trait_name,
        &preset_type,
        &ast.preset_generics,
        &ast.delegate_entries,
    );

    let item_struct = define_struct(&ast.preset_ident, &ast.preset_generics);

    let mut output = TokenStream::new();

    output.extend(item_struct.to_token_stream());

    output.extend(preset_trait.to_token_stream());

    for impl_item in impl_delegate_items {
        output.extend(impl_item.to_token_stream());
    }

    for impl_item in impl_is_reset_items {
        output.extend(impl_item.to_token_stream());
    }

    {
        let delegates_to_trait_name = format!("DelegatesTo{}", ast.preset_ident);

        let (delegates_to_trait, delegates_to_impl) = define_delegates_to_trait(
            &Ident::new(&delegates_to_trait_name, Span::call_site()),
            &preset_type,
            &ast.preset_generics,
            &ast.delegate_entries,
        );

        output.extend(delegates_to_trait.to_token_stream());
        output.extend(delegates_to_impl.to_token_stream());
    }

    {
        let with_components_macro_name =
            format!("with_{}", to_snake_case_str(&ast.preset_ident.to_string()));

        let with_components_macro = define_substitution_macro(
            &Ident::new(&with_components_macro_name, Span::call_site()),
            &ast.delegate_entries.all_components().to_token_stream(),
        );

        output.extend(with_components_macro);
    }

    Ok(output)
}
