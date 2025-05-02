use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::token::At;
use syn::{parse2, parse_quote, Ident, ItemTrait};

use crate::delegate_components::{define_struct, impl_delegate_components};
use crate::derive_component::to_snake_case_str;
use crate::parse::{DefinePreset, ImplGenerics};
use crate::preset::{define_substitution_macro, impl_components_is_preset};

pub fn define_preset(body: TokenStream) -> syn::Result<TokenStream> {
    let ast: DefinePreset = syn::parse2(body)?;

    let mut parent_presets = ast.parent_presets.clone();

    let mut remaining_parents = parent_presets
        .iter_mut()
        .filter(|parent| parent.has_expanded.is_none());

    let m_parent = if let Some(parent_preset) = remaining_parents.next() {
        parent_preset.has_expanded = Some(At(Span::call_site()));
        Some(parent_preset.parent_type.clone())
    } else {
        None
    };

    if let Some(parent) = m_parent {
        let parent_ident = &parent.name;
        let parent_generics = &parent.generics;

        let parent_components_ident = Ident::new(
            &format!("__{parent_ident}Components__"),
            parent_ident.span(),
        );

        let preset_type_spec = &ast.preset;
        let delegate_entries = &ast.delegate_entries;

        let output = quote! {
            pub use #parent_ident ::re_exports::*;

            #parent_ident :: with_components! {
                | #parent_components_ident | {
                    cgp_preset! {
                        #preset_type_spec: #parent_presets {
                            #parent_components_ident: #parent_ident :: Provider #parent_generics,
                            #delegate_entries
                        }
                    }
                }
            }
        };

        return Ok(output);
    }

    let preset_module_name = &ast.preset.name;

    let preset_generic_args = &ast.preset.generics;

    let preset_generics: ImplGenerics = syn::parse2(quote!( #preset_generic_args ))?;

    let provider_struct_name = Ident::new("Provider", Span::call_site());

    let provider_type = {
        let type_generics = preset_generics.as_type_generics();
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
        )?;

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

    let provider_struct = define_struct(&provider_struct_name, &preset_generics.generics)?;

    let mut mod_output = quote! {
        #provider_struct

        #preset_trait
    };

    mod_output.append_all(impl_is_preset_items);

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
