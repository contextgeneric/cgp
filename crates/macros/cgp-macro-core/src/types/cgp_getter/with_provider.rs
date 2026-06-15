use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Generics, Ident, ImplItem, ItemImpl, parse_quote, parse2};

use crate::types::cgp_getter::{GetterField, ItemCgpGetter, ReceiverMode};
use crate::types::getter::{ContextArg, FieldMode, derive_getter_method};
use crate::types::provider_impl::ItemProviderImpl;
use crate::visitors::get_bounds_and_replace_self_assoc_type;

impl ItemCgpGetter {
    pub fn to_with_provider_impl(&self) -> syn::Result<Option<ItemProviderImpl>> {
        if self.fields.len() == 1 {
            let field = &self.fields[0];

            let item_impl = self.derive_with_provider_impl(field)?;

            let component_type = self.item_component.args.component_name.to_type();

            let item = ItemProviderImpl {
                component_type,
                item_impl,
            };

            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    pub fn derive_with_provider_impl(&self, field: &GetterField) -> syn::Result<ItemImpl> {
        let args = &self.item_component.args;
        let provider_trait = &self.item_component.provider_trait;

        let field_assoc_type = &self.field_assoc_type;

        let component_name = &args.component_name;
        let context_type = &args.context_ident;
        let provider_name = &args.provider_ident;

        let receiver_type = match &field.receiver_mode {
            ReceiverMode::SelfReceiver => parse_quote!(#context_type),
            ReceiverMode::Type(ty) => ty.clone(),
        };

        let field_type = match field_assoc_type {
            Some(field_assoc_type) => {
                let field_assoc_type_ident = &field_assoc_type.ident;
                parse_quote! { #field_assoc_type_ident }
            }
            None => field.field_type.clone(),
        };

        let provider_ident = Ident::new("__Provider__", Span::call_site());

        let mut items: Vec<ImplItem> = Vec::new();

        let mut provider_generics = provider_trait.generics.clone();

        if let Some(field_assoc_type) = field_assoc_type {
            let field_assoc_type_ident = &field_assoc_type.ident;

            provider_generics
                .params
                .push(parse2(field_assoc_type_ident.to_token_stream())?);

            items.push(parse2(quote! {
                type #field_assoc_type_ident = #field_assoc_type_ident;
            })?);

            let field_constraints = get_bounds_and_replace_self_assoc_type(field_assoc_type);

            provider_generics
                .make_where_clause()
                .predicates
                .push(parse2(quote! {
                    #field_assoc_type_ident: #field_constraints
                })?);
        }

        let provider_constraint = if field.receiver_mut.is_none() {
            if let FieldMode::Slice = field.field_mode {
                quote! {
                    FieldGetter< #receiver_type, #component_name, Value: AsRef< [ #field_type ] > + 'static >
                }
            } else {
                quote! {
                    FieldGetter< #receiver_type, #component_name , Value = #field_type >
                }
            }
        } else {
            quote! {
                MutFieldGetter< #receiver_type, #component_name, Value = #field_type >
            }
        };

        let method = derive_getter_method(
            &ContextArg::Type(receiver_type),
            field,
            &component_name.to_type(),
            Some(provider_ident.clone()),
        )?;

        items.push(method.into());

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
                #( #items )*
            }
        })?;

        Ok(out)
    }
}
