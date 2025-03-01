use quote::{quote, ToTokens};
use syn::{parse2, Generics, ItemImpl, ItemTrait};

use crate::getter_component::getter_field::GetterField;
use crate::parse::ComponentSpec;

pub fn derive_with_provider_impl(
    spec: &ComponentSpec,
    provider_trait: &ItemTrait,
    field: &GetterField,
) -> syn::Result<ItemImpl> {
    let component_name = &spec.component_name;
    let context_type = &spec.context_type;
    let provider_name = &spec.provider_name;

    let field_name = &field.field_name;
    let provider_type = &field.provider_type;

    let provider_ident = quote! { __Provider__ };

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
                #provider_ident ::get_field(context, ::core::marker::PhantomData )
            }
        }
    } else {
        quote! {
            fn #field_name( context: &mut #context_type ) -> &mut #provider_type {
                #provider_ident ::get_field_mut(context, ::core::marker::PhantomData )
            }
        }
    };

    let mut provider_generics = provider_trait.generics.clone();

    let mut where_clause = provider_generics.make_where_clause().clone();
    where_clause
        .predicates
        .push(parse2(quote! { #provider_ident : #provider_constraint })?);

    let (impl_generics, type_generics, _) = provider_generics.split_for_impl();

    let impl_generics = {
        let mut generics: Generics = parse2(impl_generics.to_token_stream())?;
        generics.params.push(parse2(provider_ident.clone())?);
        generics
    };

    let out = parse2(quote! {
        impl #impl_generics #provider_name #type_generics for WithProvider< #provider_ident >
        #where_clause
        {
            #method
        }
    })?;

    Ok(out)
}
