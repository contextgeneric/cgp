use cgp_macro_core::types::cgp_component::CgpComponentArgs;
use cgp_macro_core::types::cgp_getter::{GetterField, ReceiverMode};
use cgp_macro_core::types::getter::{ContextArg, FieldMode};
use cgp_macro_core::visitors::get_bounds_and_replace_self_assoc_type;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Generics, Ident, ItemImpl, ItemTrait, TraitItemType, parse_quote, parse2};

use crate::derive_getter::derive_getter_method;

pub fn derive_with_provider_impl(
    spec: &CgpComponentArgs,
    provider_trait: &ItemTrait,
    field: &GetterField,
    field_assoc_type: &Option<TraitItemType>,
) -> syn::Result<ItemImpl> {
    let component_name = &spec.component_name;

    let context_type = &spec.context_ident;
    let provider_name = &spec.provider_ident;

    let receiver_type = match &field.receiver_mode {
        ReceiverMode::SelfReceiver => parse_quote!(#context_type),
        ReceiverMode::Type(ty) => ty.as_ref().clone(),
    };

    let field_type = match field_assoc_type {
        Some(field_assoc_type) => {
            let field_assoc_type_ident = &field_assoc_type.ident;
            parse_quote! { #field_assoc_type_ident }
        }
        None => field.field_type.clone(),
    };

    let provider_ident = Ident::new("__Provider__", Span::call_site());

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

    let provider_constraint = if field.receiver_mut.is_none() {
        if let FieldMode::Slice = field.field_mode {
            quote! {
                FieldGetter< #receiver_type, #component_name, Value: AsRef< [ #field_type ] > + 'static >
            }
        } else {
            quote! {
                FieldGetter< #receiver_type, #component_name , Value = #field_type >
            }
        }
    } else {
        quote! {
            MutFieldGetter< #receiver_type, #component_name, Value = #field_type >
        }
    };

    items.extend(
        derive_getter_method(
            &ContextArg::Type(receiver_type),
            field,
            &component_name.to_type(),
            Some(provider_ident.clone()),
        )?
        .to_token_stream(),
    );

    let mut where_clause = provider_generics.make_where_clause().clone();
    where_clause
        .predicates
        .push(parse2(quote! { #provider_ident : #provider_constraint })?);

    let (_, type_generics, _) = provider_trait.generics.split_for_impl();
    let (impl_generics, _, _) = provider_generics.split_for_impl();

    let impl_generics = {
        let mut generics: Generics = parse2(impl_generics.to_token_stream())?;
        generics.params.push(parse2(quote! { #provider_ident })?);
        generics
    };

    let out = parse2(quote! {
        impl #impl_generics #provider_name #type_generics for WithProvider< #provider_ident >
        #where_clause
        {
            #items
        }
    })?;

    Ok(out)
}
