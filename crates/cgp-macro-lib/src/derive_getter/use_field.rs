use cgp_macro_core::types::field::HasFieldBound;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{
    Generics, ItemImpl, ItemTrait, TraitItemType, Type, TypeParamBound, parse_quote, parse2,
};

use crate::derive_getter::getter_field::GetterField;
use crate::derive_getter::{ContextArg, ReceiverMode, derive_getter_method};
use crate::parse::ComponentSpec;
use crate::type_component::get_bounds_and_replace_self_assoc_type;

pub fn derive_use_field_impl(
    spec: &ComponentSpec,
    provider_trait: &ItemTrait,
    field: &GetterField,
    field_assoc_type: &Option<TraitItemType>,
) -> syn::Result<ItemImpl> {
    let context_type = &spec.context_type;
    let provider_name = &provider_trait.ident;

    let receiver_type = match &field.receiver_mode {
        ReceiverMode::SelfReceiver => context_type.to_token_stream(),
        ReceiverMode::Type(ty) => ty.to_token_stream(),
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

    items.extend(derive_getter_method(
        &ContextArg::Ident(receiver_type.clone()),
        field,
        None,
        None,
    ));

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
