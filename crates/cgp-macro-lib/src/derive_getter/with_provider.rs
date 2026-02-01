use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Generics, Ident, ItemImpl, ItemTrait, TraitItemType, parse_quote, parse2};

use crate::derive_getter::getter_field::GetterField;
use crate::derive_getter::{ContextArg, FieldMode, ReceiverMode, derive_getter_method};
use crate::parse::ComponentSpec;

pub fn derive_with_provider_impl(
    spec: &ComponentSpec,
    provider_trait: &ItemTrait,
    field: &GetterField,
    field_assoc_type: &Option<TraitItemType>,
) -> syn::Result<ItemImpl> {
    let component_name = &spec.component_name;
    let component_params = &spec.component_params;

    let context_type = &spec.context_type;
    let provider_name = &spec.provider_name;

    let receiver_type = match &field.receiver_mode {
        ReceiverMode::SelfReceiver => context_type.to_token_stream(),
        ReceiverMode::Type(ty) => ty.to_token_stream(),
    };

    let field_type = match field_assoc_type {
        Some(field_assoc_type) => {
            let field_assoc_type_ident = &field_assoc_type.ident;
            parse_quote! { #field_assoc_type_ident }
        }
        None => field.field_type.clone(),
    };

    let provider_ident = Ident::new("__Provider__", Span::call_site());

    let component_type = quote! { #component_name < #component_params > };

    let mut items = TokenStream::new();

    let mut provider_generics = provider_trait.generics.clone();

    if let Some(field_assoc_type) = field_assoc_type {
        let field_assoc_type_ident = &field_assoc_type.ident;

        provider_generics
            .params
            .push(parse2(field_assoc_type_ident.to_token_stream())?);

        items.extend(quote! {
            type #field_assoc_type_ident = #field_assoc_type_ident;
        });

        let field_constraints = &field_assoc_type.bounds;

        provider_generics
            .make_where_clause()
            .predicates
            .push(parse2(quote! {
                #field_assoc_type_ident: #field_constraints
            })?);
    }

    let provider_constraint = if field.field_mut.is_none() {
        if let FieldMode::Slice = field.field_mode {
            quote! {
                FieldGetter< #receiver_type, #component_type, Value: AsRef< [ #field_type ] > + 'static >
            }
        } else {
            quote! {
                FieldGetter< #receiver_type, #component_type , Value = #field_type >
            }
        }
    } else {
        quote! {
            MutFieldGetter< #receiver_type, #component_type, Value = #field_type >
        }
    };

    items.extend(derive_getter_method(
        &ContextArg::Ident(receiver_type),
        field,
        None,
        Some(provider_ident.clone()),
    ));

    let mut where_clause = provider_generics.make_where_clause().clone();
    where_clause
        .predicates
        .push(parse2(quote! { #provider_ident : #provider_constraint })?);

    let (_, type_generics, _) = provider_trait.generics.split_for_impl();
    let (impl_generics, _, _) = provider_generics.split_for_impl();

    let impl_generics = {
        let mut generics: Generics = parse2(impl_generics.to_token_stream())?;
        generics.params.push(parse2(quote! { #provider_ident })?);
        generics
    };

    let out = parse2(quote! {
        impl #impl_generics #provider_name #type_generics for WithProvider< #provider_ident >
        #where_clause
        {
            #items
        }
    })?;

    Ok(out)
}
