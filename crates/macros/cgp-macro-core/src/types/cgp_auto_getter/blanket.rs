use quote::ToTokens;
use syn::{Ident, ImplItem, ItemImpl, ItemTrait, TraitItemType};

use crate::functions::parse_internal;
use crate::types::cgp_getter::{GetterField, ReceiverMode};
use crate::types::field::{FieldName, HasFieldBound};
use crate::types::getter::{ContextArg, derive_getter_method};
use crate::visitors::get_bounds_and_replace_self_assoc_type;

pub fn derive_blanket_impl(
    context_type: &Ident,
    consumer_trait: &ItemTrait,
    fields: &[GetterField],
    field_assoc_type: &Option<TraitItemType>,
) -> syn::Result<ItemImpl> {
    let consumer_name = &consumer_trait.ident;

    let supertrait_constraints = consumer_trait.supertraits.clone();

    let mut items: Vec<ImplItem> = Vec::new();

    let mut generics = consumer_trait.generics.clone();

    generics
        .params
        .insert(0, parse_internal(context_type.to_token_stream())?);

    if let Some(field_assoc_type) = field_assoc_type {
        let field_assoc_type_ident = &field_assoc_type.ident;

        generics
            .params
            .push(parse_internal(field_assoc_type_ident.to_token_stream())?);

        items.push(parse_internal! {
            type #field_assoc_type_ident = #field_assoc_type_ident;
        });

        let field_constraints = get_bounds_and_replace_self_assoc_type(field_assoc_type);

        generics
            .make_where_clause()
            .predicates
            .push(parse_internal! {
                #field_assoc_type_ident: #field_constraints
            });
    }

    let where_clause = generics.make_where_clause();

    if !supertrait_constraints.is_empty() {
        where_clause.predicates.push(parse_internal! {
            #context_type: #supertrait_constraints
        });
    }

    for field in fields {
        let (receiver_type, context_arg) = match &field.receiver_mode {
            ReceiverMode::SelfReceiver => (context_type.to_token_stream(), ContextArg::SelfArg),
            ReceiverMode::Type(ty) => (ty.to_token_stream(), ContextArg::Type(ty.clone())),
        };

        let field_name = FieldName::from(field.field_name.clone());
        let tag_type = parse_internal!(#field_name);

        let method = derive_getter_method(&context_arg, field, &tag_type, None)?;

        items.push(method.into());

        let field_type = if let Some(trait_item) = &field_assoc_type {
            let trait_item_ident = &trait_item.ident;
            parse_internal!(#trait_item_ident)
        } else {
            field.field_type.clone()
        };

        let constraint = HasFieldBound {
            field_type,
            field_mut: field.receiver_mut,
            field_mode: field.field_mode.clone(),
            tag_type,
        };

        where_clause.predicates.push(parse_internal! {
            #receiver_type: #constraint
        });
    }

    let (_, type_generics, _) = consumer_trait.generics.split_for_impl();
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let item_impl: ItemImpl = parse_internal! {
        impl #impl_generics #consumer_name #type_generics for #context_type
        #where_clause
        {
            #( #items )*
        }
    };

    Ok(item_impl)
}
