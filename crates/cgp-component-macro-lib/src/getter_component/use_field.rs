use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemImpl, ItemTrait};

use crate::derive_component::component_spec::ComponentSpec;
use crate::getter_component::getter_field::GetterField;

pub fn derive_use_field_impl(
    spec: &ComponentSpec,
    consumer_trait: &ItemTrait,
    field: &GetterField,
) -> TokenStream {
    let context_type = &spec.context_type;
    let provider_name = &spec.provider_name;

    // FIXME: replace `Self` with `Context` inside super trait bound
    let mut constraints = consumer_trait.supertraits.clone();

    let field_name = &field.field_name;
    let provider_type = &field.provider_type;

    let method = if field.field_mut.is_none() {
        constraints.push(parse_quote! {
            HasField< Tag, Value = #provider_type >
        });

        quote! {
            fn #field_name( context: & #context_type ) -> & #provider_type {
                context.get_field( ::core::marker::PhantomData )
            }
        }
    } else {
        constraints.push(parse_quote! {
            HasFieldMut< Tag, Value = #provider_type >
        });

        quote! {
            fn #field_name( context: &mut #context_type ) -> &mut #provider_type {
                context.get_field_mut( ::core::marker::PhantomData )
            }
        }
    };

    let use_field_impl: ItemImpl = parse_quote! {
        impl< #context_type, Tag > #provider_name < #context_type > for UseField<Tag>
        where
            #context_type: #constraints
        {
            #method
        }
    };

    quote! {
        #use_field_impl
    }
}
