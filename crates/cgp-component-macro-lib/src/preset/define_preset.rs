use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse2, parse_quote, Generics, Ident, ItemTrait};

use crate::delegate_components::define_struct::define_struct;
use crate::delegate_components::delegates_to::define_delegates_to_trait;
use crate::delegate_components::impl_delegate::impl_delegate_components;
use crate::derive_component::snake_case::to_snake_case_str;
use crate::preset::ast::DefinePresetAst;
use crate::preset::impl_is_preset::impl_components_is_preset;
use crate::preset::substitution_macro::define_substitution_macro;

pub fn define_preset(body: TokenStream) -> syn::Result<TokenStream> {
    let ast: DefinePresetAst = syn::parse2(body)?;

    let preset_module_name = &ast.preset.name;

    let preset_generic_args = &ast.preset.generics;

    let preset_generics: Generics = syn::parse2(quote!( #preset_generic_args ))?;

    let provider_struct_name = Ident::new("Provider", Span::call_site());

    let provider_type = {
        let type_generics = preset_generics.split_for_impl().1;
        parse2(quote! { #provider_struct_name #type_generics })?
    };

    let preset_trait_name = Ident::new("IsPreset", Span::call_site());

    let preset_trait: ItemTrait = parse_quote! {
        #[doc(hidden)]
        pub trait #preset_trait_name <Component> {}
    };

    let impl_delegate_items = {
        let namespaces_preset_type = parse2(quote! {
            #preset_module_name :: #provider_type
        })?;

        let items = impl_delegate_components(
            &namespaces_preset_type,
            &preset_generics,
            &ast.delegate_entries,
        );

        let mut stream = TokenStream::new();
        stream.append_all(items);

        stream
    };

    let impl_is_preset_items = impl_components_is_preset(
        &preset_trait_name,
        &provider_type,
        &preset_generics,
        &ast.delegate_entries,
    );

    let provider_struct = define_struct(&provider_struct_name, &preset_generics);

    let mut mod_output = quote! {
        #provider_struct

        #preset_trait
    };

    mod_output.append_all(impl_is_preset_items);

    {
        let delegates_to_trait_name = Ident::new("DelegatesToPreset", Span::call_site());

        let (delegates_to_trait, delegates_to_impl) = define_delegates_to_trait(
            &delegates_to_trait_name,
            &provider_type,
            &preset_generics,
            &ast.delegate_entries,
        );

        mod_output.extend(delegates_to_trait.to_token_stream());
        mod_output.extend(delegates_to_impl.to_token_stream());
    }

    {
        let with_components_macro_name = Ident::new(
            &format!(
                "with_{}",
                to_snake_case_str(&preset_module_name.to_string())
            ),
            Span::call_site(),
        );

        let with_components_macro = define_substitution_macro(
            &with_components_macro_name,
            &ast.delegate_entries.all_components().to_token_stream(),
        );

        mod_output.extend(with_components_macro);
        mod_output.extend(quote! {
            pub use #with_components_macro_name as with_components;
        })
    }

    let output = quote! {
        #impl_delegate_items

        #[allow(non_snake_case)]
        pub mod #preset_module_name {
            use super::*;

            #[doc(hidden)]
            pub mod re_exports {
                #[doc(hidden)]
                #[doc(no_inline)]
                pub use super::super::super::re_exports::*;
            }

            #mod_output
        }
    };

    Ok(output)
}
