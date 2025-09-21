use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Generics, Ident, ItemImpl, ItemTrait, parse2};

use crate::derive_getter::getter_field::GetterField;
use crate::derive_getter::{ContextArg, FieldMode, derive_getter_method};
use crate::parse::ComponentSpec;

pub fn derive_with_provider_impl(
    spec: &ComponentSpec,
    provider_trait: &ItemTrait,
    field: &GetterField,
) -> syn::Result<ItemImpl> {
    let component_name = &spec.component_name;
    let component_params = &spec.component_params;

    let context_type = &spec.context_type;
    let provider_name = &spec.provider_name;

    let provider_type = &field.field_type;

    let provider_ident = Ident::new("__Provider__", Span::call_site());

    let component_type = quote! { #component_name < #component_params > };

    let provider_constraint = if field.field_mut.is_none() {
        if let FieldMode::Slice = field.field_mode {
            quote! {
                FieldGetter< #context_type, #component_type, Value: AsRef< [ #provider_type ] > + 'static >
            }
        } else {
            quote! {
                FieldGetter< #context_type, #component_type , Value = #provider_type >
            }
        }
    } else {
        quote! {
            MutFieldGetter< #context_type, #component_type, Value = #provider_type >
        }
    };

    let method = derive_getter_method(
        &ContextArg::Ident(context_type.clone()),
        field,
        None,
        Some(provider_ident.clone()),
    );

    let mut provider_generics = provider_trait.generics.clone();

    let mut where_clause = provider_generics.make_where_clause().clone();
    where_clause
        .predicates
        .push(parse2(quote! { #provider_ident : #provider_constraint })?);

    let (impl_generics, type_generics, _) = provider_generics.split_for_impl();

    let impl_generics = {
        let mut generics: Generics = parse2(impl_generics.to_token_stream())?;
        generics.params.push(parse2(quote! { #provider_ident })?);
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
