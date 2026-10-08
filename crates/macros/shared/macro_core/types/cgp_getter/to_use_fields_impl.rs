use syn::{ImplItem, ItemImpl, Type};

use crate::macro_core::exports::UseFields;
use crate::macro_core::functions::parse_internal;
use crate::macro_core::types::cgp_getter::{ItemCgpGetter, ReceiverMode};
use crate::macro_core::types::cgp_type::lift_assoc_type;
use crate::macro_core::types::field::{HasFieldBound, Symbol};
use crate::macro_core::types::getter::{ContextArg, derive_getter_method};
use crate::macro_core::types::provider_impl::ItemProviderImpl;

impl ItemCgpGetter {
    pub fn to_use_fields_impl(&self) -> syn::Result<ItemProviderImpl> {
        let provider_trait = &self.item_component.provider_trait;

        let context_type = &self.item_component.args.context_ident;

        let provider_name = &self.item_component.args.provider_ident;

        let component_name = &self.item_component.args.component_name;

        let field_assoc_type = &self.field_assoc_type;

        let mut items: Vec<ImplItem> = Vec::new();

        let mut provider_generics = provider_trait.generics.clone();

        if let Some(field_assoc_type) = &field_assoc_type {
            items.push(lift_assoc_type(
                &mut provider_generics,
                field_assoc_type,
                false,
            )?);
        }

        let where_clause = provider_generics.make_where_clause();

        for field in &self.fields {
            let receiver_type = match &field.receiver_mode {
                ReceiverMode::SelfReceiver => parse_internal!(#context_type),
                ReceiverMode::Type(ty) => ty.clone(),
            };

            let field_name = Symbol::from_ident(field.field_name.clone());
            let tag_type: Type = parse_internal!(#field_name);

            let method = derive_getter_method(
                &ContextArg::Type(receiver_type.clone()),
                field,
                &tag_type,
                None,
            )?;

            items.push(method.into());

            let field_type = if let Some(trait_item) = &field_assoc_type {
                let trait_item_ident = &trait_item.ident;
                parse_internal!(#trait_item_ident)
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
                .push(parse_internal! { #receiver_type: #constraint });
        }

        let (_, type_generics, _) = provider_trait.generics.split_for_impl();
        let (impl_generics, _, where_clause) = provider_generics.split_for_impl();

        let item_impl: ItemImpl = parse_internal! {
            impl #impl_generics #provider_name #type_generics for #UseFields
            #where_clause
            {
                #( #items )*
            }
        };

        Ok(ItemProviderImpl {
            component_type: component_name.to_type(),
            item_impl,
        })
    }
}
