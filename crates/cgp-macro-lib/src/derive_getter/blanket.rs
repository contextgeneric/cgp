use alloc::string::ToString;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, ItemImpl, ItemTrait, TraitItemType, parse2};

use crate::derive_getter::getter_field::GetterField;
use crate::derive_getter::{
    ContextArg, ReceiverMode, derive_getter_constraint, derive_getter_method,
};
use crate::symbol::symbol_from_string;

pub fn derive_blanket_impl(
    context_type: &Ident,
    consumer_trait: &ItemTrait,
    fields: &[GetterField],
    field_assoc_type: &Option<TraitItemType>,
) -> syn::Result<ItemImpl> {
    let consumer_name = &consumer_trait.ident;

    let supertrait_constraints = consumer_trait.supertraits.clone();

    let mut items: TokenStream = TokenStream::new();

    let mut generics = consumer_trait.generics.clone();

    generics
        .params
        .insert(0, parse2(context_type.to_token_stream())?);

    if let Some(field_assoc_type) = field_assoc_type {
        let field_assoc_type_ident = &field_assoc_type.ident;

        generics
            .params
            .push(parse2(field_assoc_type_ident.to_token_stream())?);

        items.extend(quote! {
            type #field_assoc_type_ident = #field_assoc_type_ident;
        });

        let field_constraints = &field_assoc_type.bounds;

        generics.make_where_clause().predicates.push(parse2(quote! {
            #field_assoc_type_ident: #field_constraints
        })?);
    }

    let where_clause = generics.make_where_clause();

    if !supertrait_constraints.is_empty() {
        where_clause.predicates.push(parse2(quote! {
            #context_type: #supertrait_constraints
        })?);
    }

    for field in fields {
        let (receiver_type, context_arg) = match &field.receiver_mode {
            ReceiverMode::SelfReceiver => (context_type.to_token_stream(), ContextArg::SelfArg),
            ReceiverMode::Type(ty) => (
                ty.to_token_stream(),
                ContextArg::Ident(ty.to_token_stream()),
            ),
        };

        let field_symbol = symbol_from_string(&field.field_name.to_string());

        let method = derive_getter_method(
            &context_arg,
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

        where_clause.predicates.push(parse2(quote! {
            #receiver_type: #constraint
        })?);
    }

    let (_, type_generics, _) = consumer_trait.generics.split_for_impl();
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let item_impl: ItemImpl = parse2(quote! {
        impl #impl_generics #consumer_name #type_generics for #context_type
        #where_clause
        {
            #items
        }
    })?;

    Ok(item_impl)
}
