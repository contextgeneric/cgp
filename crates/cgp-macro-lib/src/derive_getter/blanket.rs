use alloc::string::ToString;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, ItemImpl, ItemTrait, parse2};

use crate::derive_getter::getter_field::GetterField;
use crate::derive_getter::{
    ContextArg, ReceiverMode, derive_getter_constraint, derive_getter_method,
};
use crate::symbol::symbol_from_string;

pub fn derive_blanket_impl(
    context_type: &Ident,
    consumer_trait: &ItemTrait,
    fields: &[GetterField],
) -> syn::Result<ItemImpl> {
    let consumer_name = &consumer_trait.ident;

    let supertrait_constraints = consumer_trait.supertraits.clone();

    let mut methods: TokenStream = TokenStream::new();

    let mut generics = consumer_trait.generics.clone();

    generics
        .params
        .insert(0, parse2(context_type.to_token_stream())?);

    let where_clause = generics.make_where_clause();
    where_clause.predicates.push(parse2(quote! {
        #context_type: #supertrait_constraints
    })?);

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

        methods.extend(method);

        let constraint = derive_getter_constraint(field, quote! { #field_symbol })?;

        where_clause.predicates.push(parse2(quote! {
            #receiver_type: #constraint
        })?);
    }

    let (_, type_generics, _) = consumer_trait.generics.split_for_impl();
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let mut item_impl: ItemImpl = parse2(quote! {
        impl #impl_generics #consumer_name #type_generics for #context_type
        #where_clause
        {
            #methods
        }
    })?;

    item_impl
        .generics
        .params
        .extend(consumer_trait.generics.params.clone());

    if let Some(consumer_where_clause) = &consumer_trait.generics.where_clause {
        item_impl
            .generics
            .make_where_clause()
            .predicates
            .extend(consumer_where_clause.predicates.clone());
    }

    Ok(item_impl)
}
