use cgp_macro_core::types::attributes::CgpComponentAttributes;
use cgp_macro_core::types::cgp_component::CgpComponentArgs;
use cgp_macro_core::types::empty_struct::EmptyStruct;
use cgp_macro_core::types::is_provider_for::derive_is_provider_for;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{ItemImpl, ItemTrait, parse2};

use crate::derive_component::consumer_impl::derive_consumer_impl;
use crate::derive_component::derive_namespace::derive_namespace_impls;
use crate::derive_component::derive_redirect_lookup::derive_redirect_lookup_impl;
use crate::derive_component::preprocess_consumer_trait;
use crate::derive_component::provider_impl::derive_provider_impl;
use crate::derive_component::provider_trait::derive_provider_trait;
use crate::derive_component::use_context_impl::derive_use_context_impl;
use crate::derive_component::use_delegate_impl::derive_delegate_impl;

pub fn derive_component_with_ast(
    spec: &CgpComponentArgs,
    mut consumer_trait: ItemTrait,
) -> syn::Result<DerivedComponent> {
    let provider_name = &spec.provider_ident;
    let context_type = &spec.context_ident;

    let component_name = &spec.component_name;

    let attributes = CgpComponentAttributes::parse(&mut consumer_trait.attrs)?;

    preprocess_consumer_trait(&mut consumer_trait, &attributes)?;

    let component_struct = EmptyStruct {
        ident: component_name.ident.clone(),
        generics: component_name.type_generics.generics.clone(),
    };

    let provider_trait =
        derive_provider_trait(component_name, &consumer_trait, provider_name, context_type)?;

    let consumer_impl = derive_consumer_impl(&consumer_trait, provider_name, context_type)?;

    let provider_impl = derive_provider_impl(
        context_type,
        &consumer_trait,
        &provider_trait,
        component_name,
    )?;

    let use_context_impl = derive_use_context_impl(context_type, &consumer_trait, &provider_trait)?;

    let use_context_is_provider_impl = derive_is_provider_for(
        &parse2(quote! {
            #component_name
        })?,
        &use_context_impl,
    )?;

    let redirect_lookup_impl = derive_redirect_lookup_impl(&consumer_trait, &provider_trait)?;
    let redirect_lookup_is_provider_impl = derive_is_provider_for(
        &parse2(quote! {
            #component_name
        })?,
        &redirect_lookup_impl,
    )?;

    let mut item_impls = vec![
        provider_impl,
        consumer_impl,
        use_context_impl,
        use_context_is_provider_impl,
        redirect_lookup_impl,
        redirect_lookup_is_provider_impl,
    ];

    if !spec.derive_delegate_attributes.attributes.is_empty() {
        for spec in spec.derive_delegate_attributes.attributes.iter() {
            let use_delegate_impl = derive_delegate_impl(&provider_trait, spec)?;

            let use_delegate_is_provider_impl = derive_is_provider_for(
                &parse2(quote! {
                    #component_name
                })?,
                &use_delegate_impl,
            )?;

            item_impls.push(use_delegate_impl);
            item_impls.push(use_delegate_is_provider_impl);
        }
    }

    let namespace_impls = derive_namespace_impls(&attributes.prefixes, &component_name.ident)?;
    item_impls.extend(namespace_impls);

    let derived = DerivedComponent {
        component_struct,
        consumer_trait,
        provider_trait,
        item_impls,
    };

    Ok(derived)
}

pub struct DerivedComponent {
    pub component_struct: EmptyStruct,
    pub consumer_trait: ItemTrait,
    pub provider_trait: ItemTrait,
    pub item_impls: Vec<ItemImpl>,
}

impl ToTokens for DerivedComponent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.component_struct.to_tokens(tokens);
        self.consumer_trait.to_tokens(tokens);
        self.provider_trait.to_tokens(tokens);
        tokens.append_all(self.item_impls.iter());
    }
}
