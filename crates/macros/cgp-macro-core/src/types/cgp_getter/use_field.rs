use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{Generics, ItemImpl, Type, TypeParamBound, parse_quote, parse2};

use crate::types::cgp_getter::{GetterField, ItemCgpGetter, ReceiverMode};
use crate::types::field::HasFieldBound;
use crate::types::getter::{ContextArg, derive_getter_method};
use crate::types::provider_impl::ItemProviderImpl;
use crate::visitors::get_bounds_and_replace_self_assoc_type;

impl ItemCgpGetter {
    pub fn to_use_field_impl(&self) -> syn::Result<Option<ItemProviderImpl>> {
        if self.fields.len() == 1 {
            let field = &self.fields[0];

            let item_impl = self.derive_use_field_impl(field)?;

            let component_type = self.item_component.args.component_name.to_type();

            let item = ItemProviderImpl {
                component_type,
                item_impl,
            };

            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    fn derive_use_field_impl(&self, field: &GetterField) -> syn::Result<ItemImpl> {
        let context_type = &self.item_component.args.context_ident;
        let provider_trait = &self.item_component.provider_trait;
        let field_assoc_type = &self.field_assoc_type;
        let provider_name = &provider_trait.ident;

        let receiver_type = match &field.receiver_mode {
            ReceiverMode::SelfReceiver => parse_quote!(#context_type),
            ReceiverMode::Type(ty) => ty.clone(),
        };

        let mut field_constraints: Punctuated<TypeParamBound, Plus> = Punctuated::default();

        let tag_type: Type = parse_quote! { __Tag__ };

        let mut items = TokenStream::new();

        let mut provider_generics = provider_trait.generics.clone();

        if let Some(field_assoc_type) = field_assoc_type {
            let field_assoc_type_ident = &field_assoc_type.ident;

            provider_generics
                .params
                .push(parse2(field_assoc_type_ident.to_token_stream())?);

            items.extend(quote! {
                type #field_assoc_type_ident = #field_assoc_type_ident;
            });

            let field_constraints = get_bounds_and_replace_self_assoc_type(field_assoc_type);

            provider_generics
                .make_where_clause()
                .predicates
                .push(parse2(quote! {
                    #field_assoc_type_ident: #field_constraints
                })?);
        }

        items.extend(
            derive_getter_method(
                &ContextArg::Type(receiver_type.clone()),
                field,
                &tag_type,
                None,
            )?
            .to_token_stream(),
        );

        let field_type = if let Some(trait_item) = &field_assoc_type {
            let trait_item_ident = &trait_item.ident;
            parse_quote!(#trait_item_ident)
        } else {
            field.field_type.clone()
        };

        let constraint = HasFieldBound {
            field_type,
            field_mut: field.receiver_mut,
            field_mode: field.field_mode.clone(),
            tag_type: tag_type.clone(),
        };

        field_constraints.push(parse_quote!(#constraint));

        let mut where_clause = provider_generics.make_where_clause().clone();
        where_clause
            .predicates
            .push(parse2(quote! { #receiver_type: #field_constraints })?);

        let (_, type_generics, _) = provider_trait.generics.split_for_impl();
        let (impl_generics, _, _) = provider_generics.split_for_impl();

        let impl_generics = {
            let mut generics: Generics = parse2(impl_generics.to_token_stream())?;
            generics.params.push(parse_quote!(#tag_type));
            generics
        };

        let use_field_impl: ItemImpl = parse2(quote! {
            impl #impl_generics #provider_name #type_generics for UseField< #tag_type >
            #where_clause
            {
                #items
            }
        })?;

        Ok(use_field_impl)
    }
}
