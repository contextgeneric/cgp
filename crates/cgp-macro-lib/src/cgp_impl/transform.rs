use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::token::For;
use syn::visit_mut::VisitMut;
use syn::{Ident, ImplItem, ItemImpl, Type, parse2};

use crate::parse::SimpleType;
use crate::replace_self::{
    ReplaceSelfReceiverVisitor, ReplaceSelfTypeVisitor, ReplaceSelfValueVisitor,
    to_snake_case_ident,
};

pub fn transform_impl_trait(
    item_impl: &ItemImpl,
    consumer_trait_path: &SimpleType,
    provider_type: &Type,
    context_type: &Type,
) -> syn::Result<ItemImpl> {
    let context_ident = if let Ok(ident) = parse2::<Ident>(context_type.to_token_stream()) {
        to_snake_case_ident(&ident)
    } else {
        Ident::new("__context__", Span::call_site())
    };

    let local_assoc_types: Vec<Ident> = item_impl
        .items
        .iter()
        .filter_map(|item| {
            if let ImplItem::Type(assoc_type) = item {
                Some(assoc_type.ident.clone())
            } else {
                None
            }
        })
        .collect();

    let mut replace_self_type_visitor = ReplaceSelfTypeVisitor {
        replaced_type: &context_type,
        skip_assoc_types: &local_assoc_types,
    };

    let mut out_impl = item_impl.clone();

    replace_self_type_visitor.visit_item_impl_mut(&mut out_impl);

    ReplaceSelfReceiverVisitor {
        replaced_ident: &context_ident,
        replaced_type: &context_type,
    }
    .visit_item_impl_mut(&mut out_impl);

    ReplaceSelfValueVisitor {
        replaced_ident: &context_ident,
    }
    .visit_item_impl_mut(&mut out_impl);

    out_impl.self_ty = Box::new(provider_type.clone());

    let mut provider_trait_path = consumer_trait_path.clone();
    if let Some(generics) = &mut provider_trait_path.generics {
        replace_self_type_visitor.visit_angle_bracketed_generic_arguments_mut(generics);
    }

    match &mut provider_trait_path.generics {
        Some(generics) => {
            generics
                .args
                .insert(0, parse2(context_type.to_token_stream())?);
        }
        None => {
            provider_trait_path.generics = Some(parse2(quote! { < #context_type > })?);
        }
    }

    out_impl.trait_ = Some((
        None,
        parse2(provider_trait_path.to_token_stream())?,
        For(Span::call_site()),
    ));

    Ok(out_impl)
}
