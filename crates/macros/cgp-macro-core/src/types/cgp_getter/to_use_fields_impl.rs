use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Type, parse_quote, parse2};

use crate::types::cgp_getter::{ItemCgpGetter, ReceiverMode};
use crate::types::field::Symbol;
use crate::types::provider_impl::ItemProviderImpl;
use crate::visitors::get_bounds_and_replace_self_assoc_type;


impl ItemCgpGetter {
    pub fn to_use_fields_impl(&self) -> syn::Result<ItemProviderImpl> {
        let provider_trait = &self.item_component.provider_trait;

        let context_type = &self.item_component.args.context_ident;

        let provider_name = &self.item_component.args.provider_ident;

        let mut items: TokenStream = TokenStream::new();

        let mut provider_generics = provider_trait.generics.clone();

        if let Some(field_assoc_type) = &self.field_assoc_type {
            let field_assoc_type_ident = &field_assoc_type.ident;

            provider_generics
                .params
                .push(parse2(field_assoc_type_ident.to_token_stream())?);

            items.extend(quote! {
                type #field_assoc_type_ident = #field_assoc_type_ident;
            });

            let field_constraints = get_bounds_and_replace_self_assoc_type(field_assoc_type);

            provider_generics
                .make_where_clause()
                .predicates
                .push(parse2(quote! {
                    #field_assoc_type_ident: #field_constraints
                })?);
        }

        let where_clause = provider_generics.make_where_clause();

        for field in &self.fields {
            let receiver_type = match &field.receiver_mode {
                ReceiverMode::SelfReceiver => context_type.to_token_stream(),
                ReceiverMode::Type(ty) => ty.to_token_stream(),
            };

            let field_name = Symbol::new(field.field_name.clone());
            let tag_type: Type = parse_quote!(#field_name);

            let method = derive_getter_method(
                &ContextArg::Ident(receiver_type.clone()),
                field,
                Some(quote! { ::< #field_name > }),
                None,
            );

            items.extend(method);

            let field_type = if let Some(trait_item) = &field_assoc_type {
                let trait_item_ident = &trait_item.ident;
                parse_quote!(#trait_item_ident)
            } else {
                field.field_type.clone()
            };

            let constraint = HasFieldBound {
                field_type,
                field_mut: field.receiver_mut,
                field_mode: field.field_mode.clone(),
                tag_type: tag_type.clone(),
            };

            where_clause
                .predicates
                .push(parse2(quote! { #receiver_type: #constraint })?);
        }

        let (_, type_generics, _) = provider_trait.generics.split_for_impl();
        let (impl_generics, _, where_clause) = provider_generics.split_for_impl();

        let out = parse2(quote! {
            impl #impl_generics #provider_name #type_generics for UseFields
            #where_clause
            {
                #items
            }
        })?;
        todo!()
    }
}
