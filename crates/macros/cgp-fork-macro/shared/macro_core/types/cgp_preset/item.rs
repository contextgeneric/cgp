use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Plus};
use syn::{Ident, Path, Type, braced};

use crate::macro_core::functions::to_snake_case_str;
use crate::macro_core::types::delegate_component::{
    DelegateEntries, EvalDelegateEntries, ValidateAttributes, define_with_components_macro,
    invoke_with_components,
};
use crate::macro_core::types::empty_struct::EmptyStruct;

/// `cgp_preset! { Name: Parent + Other { entries } }`.
///
/// The name is a module whose `Provider` implements `DelegateComponent` for
/// every entry. `with_components!` replays those keys, and each parent is
/// forwarded so two presets combine into one.
pub struct ItemCgpPreset {
    pub name: Ident,
    pub parents: Vec<Path>,
    pub entries: DelegateEntries,
}

impl Parse for ItemCgpPreset {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;

        let parents = if input.peek(Colon) {
            let _: Colon = input.parse()?;
            Punctuated::<Path, Plus>::parse_separated_nonempty(input)?
                .into_iter()
                .collect()
        } else {
            Vec::new()
        };

        let entries = {
            let content;
            braced!(content in input);
            content.parse()?
        };

        Ok(Self {
            name,
            parents,
            entries,
        })
    }
}

impl ItemCgpPreset {
    pub fn expand(&self) -> syn::Result<TokenStream> {
        self.entries.validate_attributes()?;

        let name = &self.name;
        let provider_ty: Type = syn::parse_quote!(#name::Provider);
        let own_impls = self
            .entries
            .build_impls(&provider_ty, &Default::default())?;

        let own_keys = self
            .entries
            .eval_entries(&provider_ty)?
            .into_iter()
            .map(|entry| entry.key)
            .collect::<Vec<_>>();

        let parent_impls = self.parents.iter().map(|parent| {
            invoke_with_components(parent, &provider_ty, &Default::default(), &own_keys)
        });

        let macro_name = Ident::new(
            &format!("__cgp_with_{}", to_snake_case_str(&self.name.to_string())),
            Span::call_site(),
        );
        let with_components = define_with_components_macro(&macro_name, &own_keys, &self.parents);

        let provider = EmptyStruct {
            ident: Ident::new("Provider", Span::call_site()),
            generics: Default::default(),
        };

        let name = &self.name;

        Ok(quote! {
            #(#own_impls)*

            #(#parent_impls)*

            #[allow(non_snake_case)]
            pub mod #name {
                #provider

                #with_components

                pub use #macro_name as with_components;
            }
        })
    }
}
