use alloc::string::ToString;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Ident, ItemImpl, ItemTrait};

use crate::getter_component::getter_field::GetterField;
use crate::getter_component::symbol::symbol_from_string;

pub fn derive_blanket_impl(
    context_type: &Ident,
    consumer_trait: &ItemTrait,
    fields: &[GetterField],
) -> ItemImpl {
    let consumer_name = &consumer_trait.ident;

    let mut constraints = consumer_trait.supertraits.clone();

    let mut methods: TokenStream = TokenStream::new();

    for field in fields {
        let field_name = &field.field_name;
        let provider_type = &field.provider_type;
        let field_symbol = symbol_from_string(&field.field_name.to_string());

        if field.field_mut.is_none() {
            constraints.push(parse_quote! {
                HasField< #field_symbol, Value = #provider_type >
            });

            methods.extend(quote! {
                fn #field_name( &self ) -> & #provider_type {
                    self.get_field( ::core::marker::PhantomData::< #field_symbol > )
                }
            });
        } else {
            constraints.push(parse_quote! {
                HasFieldMut< #field_symbol, Value = #provider_type >
            });

            methods.extend(quote! {
                fn #field_name( &mut self ) -> &mut #provider_type {
                    self.get_field_mut( ::core::marker::PhantomData::< #field_symbol > )
                }
            });
        }
    }

    let (_, type_generics, _) = consumer_trait.generics.split_for_impl();

    let mut item_impl: ItemImpl = parse_quote! {
        impl< #context_type > #consumer_name #type_generics for #context_type
        where
            #context_type: #constraints
        {
            #methods
        }
    };

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

    item_impl
}
