use alloc::string::ToString;

use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{parse2, ItemImpl, ItemTrait, TypeParamBound};

use crate::getter_component::getter_field::GetterField;
use crate::getter_component::symbol::symbol_from_string;
use crate::getter_component::{derive_getter_constraint, derive_getter_method, ContextArg};
use crate::parse::ComponentSpec;

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
        let field_symbol = symbol_from_string(&field.field_name.to_string());

        let method = derive_getter_method(
            &ContextArg::Ident(context_type.clone()),
            field,
            Some(quote! { ::< #field_symbol > }),
            None,
        );

        methods.extend(method);

        let constraint = derive_getter_constraint(field, quote! { #field_symbol })?;

        field_constraints.push(constraint);
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
