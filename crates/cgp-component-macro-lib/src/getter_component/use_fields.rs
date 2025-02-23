use alloc::string::ToString;

use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{parse2, parse_quote, ItemImpl, ItemTrait, TypeParamBound};

use crate::derive_component::component_spec::ComponentSpec;
use crate::getter_component::getter_field::GetterField;
use crate::getter_component::symbol::symbol_from_string;

pub fn derive_use_fields_impl(
    spec: &ComponentSpec,
    provider_trait: &ItemTrait,
    fields: &[GetterField],
) -> syn::Result<ItemImpl> {
    let context_type = &spec.context_type;
    let provider_name = &spec.provider_name;

    // FIXME: replace `Self` with `Context` inside super trait bound
    let mut field_constraints: Punctuated<TypeParamBound, Plus> = Punctuated::default();

    let mut methods: TokenStream = TokenStream::new();

    for field in fields {
        let field_name = &field.field_name;
        let provider_type = &field.provider_type;
        let field_symbol = symbol_from_string(&field.field_name.to_string());

        if field.field_mut.is_none() {
            field_constraints.push(parse_quote! {
                HasField< #field_symbol, Value = #provider_type >
            });

            methods.extend(quote! {
                fn #field_name( context: & #context_type ) -> & #provider_type {
                    context.get_field( ::core::marker::PhantomData::< #field_symbol > )
                }
            });
        } else {
            field_constraints.push(parse_quote! {
                HasFieldMut< #field_symbol, Value = #provider_type >
            });

            methods.extend(quote! {
                fn #field_name( context: &mut #context_type ) -> &mut #provider_type {
                    context.get_field_mut( ::core::marker::PhantomData::< #field_symbol > )
                }
            });
        }
    }

    let mut provider_generics = provider_trait.generics.clone();

    let mut where_clause = provider_generics.make_where_clause().clone();
    where_clause
        .predicates
        .push(parse2(quote! { #context_type: #field_constraints })?);

    let (impl_generics, type_generics, _) = provider_generics.split_for_impl();

    let out = parse2(quote! {
        impl #impl_generics #provider_name #type_generics for UseFields
        #where_clause
        {
            #methods
        }
    })?;

    Ok(out)
}
