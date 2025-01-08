use alloc::string::ToString;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemImpl, ItemTrait};

use crate::derive_component::component_spec::ComponentSpec;
use crate::getter_component::getter_field::GetterField;
use crate::getter_component::symbol::symbol_from_string;

pub fn derive_use_fields_impl(
    spec: &ComponentSpec,
    consumer_trait: &ItemTrait,
    fields: &[GetterField],
) -> ItemImpl {
    let context_type = &spec.context_type;
    let provider_name = &spec.provider_name;

    // FIXME: replace `Self` with `Context` inside super trait bound
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
                fn #field_name( context: & #context_type ) -> & #provider_type {
                    context.get_field( ::core::marker::PhantomData::< #field_symbol > )
                }
            });
        } else {
            constraints.push(parse_quote! {
                HasFieldMut< #field_symbol, Value = #provider_type >
            });

            methods.extend(quote! {
                fn #field_name( context: &mut #context_type ) -> &mut #provider_type {
                    context.get_field_mut( ::core::marker::PhantomData::< #field_symbol > )
                }
            });
        }
    }

    parse_quote! {
        impl< #context_type > #provider_name < #context_type > for UseFields
        where
            #context_type: #constraints
        {
            #methods
        }
    }
}
