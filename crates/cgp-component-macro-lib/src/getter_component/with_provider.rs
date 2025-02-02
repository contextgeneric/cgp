use quote::quote;
use syn::{parse_quote, ItemImpl, ItemTrait};

use crate::derive_component::component_spec::ComponentSpec;
use crate::getter_component::getter_field::GetterField;

pub fn derive_with_provider_impl(
    spec: &ComponentSpec,
    consumer_trait: &ItemTrait,
    field: &GetterField,
) -> ItemImpl {
    let component_name = &spec.component_name;
    let context_type = &spec.context_type;
    let provider_name = &spec.provider_name;

    // FIXME: replace `Self` with `Context` inside super trait bound
    let context_constraints = consumer_trait.supertraits.clone();

    let field_name = &field.field_name;
    let provider_type = &field.provider_type;

    let provider_constraint = if field.field_mut.is_none() {
        quote! {
            FieldGetter< #context_type, #component_name, Value = #provider_type >
        }
    } else {
        quote! {
            MutFieldGetter< #context_type, #component_name, Value = #provider_type >
        }
    };

    let method = if field.field_mut.is_none() {
        quote! {
            fn #field_name( context: & #context_type ) -> & #provider_type {
                Provider::get_field(context, ::core::marker::PhantomData )
            }
        }
    } else {
        quote! {
            fn #field_name( context: &mut #context_type ) -> &mut #provider_type {
                Provider::get_field_mut(context, ::core::marker::PhantomData )
            }
        }
    };

    parse_quote! {
        impl< #context_type, Provider > #provider_name < #context_type > for WithProvider<Provider>
        where
            #context_type: #context_constraints,
            Provider: #provider_constraint,
        {
            #method
        }
    }
}
