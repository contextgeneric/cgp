use alloc::string::ToString;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{ItemImpl, ItemTrait, parse2};

use crate::derive_getter::getter_field::GetterField;
use crate::derive_getter::{
    ContextArg, ReceiverMode, derive_getter_constraint, derive_getter_method,
};
use crate::parse::ComponentSpec;
use crate::symbol::symbol_from_string;

pub fn derive_use_fields_impl(
    spec: &ComponentSpec,
    provider_trait: &ItemTrait,
    fields: &[GetterField],
) -> syn::Result<ItemImpl> {
    let context_type = &spec.context_type;

    let provider_name = &spec.provider_name;

    let mut methods: TokenStream = TokenStream::new();

    let mut provider_generics = provider_trait.generics.clone();
    let mut where_clause = provider_generics.make_where_clause().clone();

    for field in fields {
        let receiver_type = match &field.receiver_mode {
            ReceiverMode::SelfReceiver => context_type.to_token_stream(),
            ReceiverMode::Type(ty) => ty.to_token_stream(),
        };

        let field_symbol = symbol_from_string(&field.field_name.to_string());

        let method = derive_getter_method(
            &ContextArg::Ident(receiver_type.clone()),
            field,
            Some(quote! { ::< #field_symbol > }),
            None,
        );

        methods.extend(method);

        let constraint = derive_getter_constraint(field, quote! { #field_symbol })?;

        where_clause
            .predicates
            .push(parse2(quote! { #receiver_type: #constraint })?);
    }

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
