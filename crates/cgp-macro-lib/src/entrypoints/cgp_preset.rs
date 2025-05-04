use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::token::{At, Comma};
use syn::{parse2, parse_quote, Ident, ItemTrait};

use crate::delegate_components::{define_struct, impl_delegate_components};
use crate::derive_component::to_snake_case_str;
use crate::parse::{DefinePreset, DelegateEntry, ImplGenerics, SimpleType};
use crate::preset::{define_substitution_macro, impl_components_is_preset};

pub fn define_preset(body: TokenStream) -> syn::Result<TokenStream> {
    let ast: DefinePreset = syn::parse2(body)?;

    let delegate_entries: Punctuated<DelegateEntry<SimpleType>, Comma> = ast
        .delegate_entries
        .iter()
        .map(|entry| entry.entry.clone())
        .collect();

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

        let mut overrides: Punctuated<&Ident, Comma> = Punctuated::default();

        for entry in ast.delegate_entries.iter() {
            if entry.is_override.is_some() {
                for component in entry.entry.keys.iter() {
                    overrides.push(&component.ty.name);
                }
            }
        }

        let filter = if !overrides.is_empty() {
            quote! {
                [ #overrides ],
            }
        } else {
            TokenStream::new()
        };

        let preset_entries = &ast.delegate_entries;

        let output = quote! {
            use #parent_ident ::components::*;

            #parent_ident :: with_components! {
                #filter
                | #parent_components_ident | {
                    cgp_preset! {
                        #preset_type_spec: #parent_presets {
                            #parent_components_ident: #parent_ident :: Provider #parent_generics,
                            #preset_entries
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

        let items =
            impl_delegate_components(&namespaces_preset_type, &preset_generics, &delegate_entries)?;

        let mut stream = TokenStream::new();
        stream.append_all(items);

        stream
    };

    let impl_is_preset_items = impl_components_is_preset(
        &preset_trait_name,
        &provider_type,
        &preset_generics,
        &delegate_entries,
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

        let all_components: Punctuated<_, Comma> = delegate_entries
            .iter()
            .flat_map(|entry| entry.keys.clone().into_iter())
            .collect();

        let with_components_macro = define_substitution_macro(
            &with_components_macro_name,
            &all_components.to_token_stream(),
        );

        mod_output.extend(with_components_macro);
        mod_output.extend(quote! {
            pub use #with_components_macro_name as with_components;
        })
    }

    let re_exports_mod = {
        let mut parent_exports = TokenStream::new();

        for parent in parent_presets.iter() {
            let parent_ident = &parent.parent_type.name;
            parent_exports.append_all(quote! {
                #[doc(hidden)]
                #[doc(no_inline)]
                pub use super:: #parent_ident ::components::*;
            });
        }

        quote! {
            #[doc(hidden)]
            #[allow(unused_imports)]
            mod re_exports {
                #[doc(hidden)]
                #[doc(no_inline)]
                pub use super::super::super::re_exports::*;

                #[doc(hidden)]
                #[doc(no_inline)]
                pub use super::super::*;

                #parent_exports
            }
        }
    };

    let components_mod = {
        let mut components: HashSet<Ident> = HashSet::default();

        for entry in delegate_entries.iter() {
            for component in entry.keys.iter() {
                let component_name = &component.ty.name;
                components.insert(component_name.clone());
            }
        }

        let components_list: Punctuated<Ident, Comma> = Punctuated::from_iter(components);

        quote! {
            #[doc(hidden)]
            pub mod components {
                #[doc(hidden)]
                #[doc(no_inline)]
                pub use super::re_exports::{ #components_list };
            }
        }
    };

    mod_output.append_all(re_exports_mod);
    mod_output.append_all(components_mod);

    let output = quote! {
        #impl_delegate_items

        #[allow(non_snake_case)]
        pub mod #preset_module_name {
            use super::*;

            #mod_output
        }
    };

    Ok(output)
}
