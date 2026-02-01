use alloc::string::ToString;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{ItemImpl, ItemTrait, TraitItemType, parse2};

use crate::derive_getter::getter_field::GetterField;
use crate::derive_getter::{
    ContextArg, ReceiverMode, derive_getter_constraint, derive_getter_method,
};
use crate::parse::ComponentSpec;
use crate::symbol::symbol_from_string;

pub fn derive_use_fields_impl(
    spec: &ComponentSpec,
    provider_trait: &ItemTrait,
    fields: &[GetterField],
    field_assoc_type: &Option<TraitItemType>,
) -> syn::Result<ItemImpl> {
    let context_type = &spec.context_type;

    let provider_name = &spec.provider_name;

    let mut items: TokenStream = TokenStream::new();

    let mut provider_generics = provider_trait.generics.clone();

    if let Some(field_assoc_type) = field_assoc_type {
        let field_assoc_type_ident = &field_assoc_type.ident;

        provider_generics
            .params
            .push(parse2(field_assoc_type_ident.to_token_stream())?);

        items.extend(quote! {
            type #field_assoc_type_ident = #field_assoc_type_ident;
        });

        let field_constraints = &field_assoc_type.bounds;

        provider_generics
            .make_where_clause()
            .predicates
            .push(parse2(quote! {
                #field_assoc_type_ident: #field_constraints
            })?);
    }

    let where_clause = provider_generics.make_where_clause();

    for field in fields {
        let receiver_type = match &field.receiver_mode {
            ReceiverMode::SelfReceiver => context_type.to_token_stream(),
            ReceiverMode::Type(ty) => ty.to_token_stream(),
        };

        let field_symbol = symbol_from_string(&field.field_name.to_string());

        let method = derive_getter_method(
            &ContextArg::Ident(receiver_type.clone()),
            field,
            Some(quote! { ::< #field_symbol > }),
            None,
        );

        items.extend(method);

        let constraint = derive_getter_constraint(
            field,
            quote! { #field_symbol },
            &field_assoc_type.as_ref().map(|item| item.ident.clone()),
        )?;

        where_clause
            .predicates
            .push(parse2(quote! { #receiver_type: #constraint })?);
    }

    let (_, type_generics, _) = provider_trait.generics.split_for_impl();
    let (impl_generics, _, where_clause) = provider_generics.split_for_impl();

    let out = parse2(quote! {
        impl #impl_generics #provider_name #type_generics for UseFields
        #where_clause
        {
            #items
        }
    })?;

    Ok(out)
}
