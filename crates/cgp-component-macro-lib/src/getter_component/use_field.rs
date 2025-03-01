use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{parse2, Generics, ItemImpl, ItemTrait, TypeParamBound};

use crate::getter_component::getter_field::GetterField;
use crate::parse::ComponentSpec;

pub fn derive_use_field_impl(
    spec: &ComponentSpec,
    provider_trait: &ItemTrait,
    field: &GetterField,
) -> syn::Result<ItemImpl> {
    let context_type = &spec.context_type;
    let provider_name = &provider_trait.ident;

    let mut field_constraints: Punctuated<TypeParamBound, Plus> = Punctuated::default();

    let field_name = &field.field_name;
    let provider_type = &field.provider_type;

    let tag_type = quote! { __Tag__ };

    let method = if field.field_mut.is_none() {
        field_constraints.push(parse2(quote! {
            HasField< #tag_type, Value = #provider_type >
        })?);

        quote! {
            fn #field_name( context: & #context_type ) -> & #provider_type {
                context.get_field( ::core::marker::PhantomData )
            }
        }
    } else {
        field_constraints.push(parse2(quote! {
            HasFieldMut< #tag_type, Value = #provider_type >
        })?);

        quote! {
            fn #field_name( context: &mut #context_type ) -> &mut #provider_type {
                context.get_field_mut( ::core::marker::PhantomData )
            }
        }
    };

    let mut provider_generics = provider_trait.generics.clone();

    let mut where_clause = provider_generics.make_where_clause().clone();
    where_clause
        .predicates
        .push(parse2(quote! { #context_type: #field_constraints })?);

    let (impl_generics, type_generics, _) = provider_generics.split_for_impl();

    let impl_generics = {
        let mut generics: Generics = parse2(impl_generics.to_token_stream())?;
        generics.params.push(parse2(tag_type.clone())?);
        generics
    };

    let use_field_impl: ItemImpl = parse2(quote! {
        impl #impl_generics #provider_name #type_generics for UseField< #tag_type >
        #where_clause
        {
            #method
        }
    })?;

    Ok(use_field_impl)
}
