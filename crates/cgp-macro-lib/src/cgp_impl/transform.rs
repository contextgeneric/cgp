use cgp_macro_core::functions::to_snake_case_ident;
use cgp_macro_core::types::ident::IdentWithTypeArgs;
use cgp_macro_core::visitors::{
    ReplaceSelfReceiverVisitor, ReplaceSelfTypeVisitor, ReplaceSelfValueVisitor,
};
use proc_macro2::Span;
use quote::ToTokens;
use syn::token::For;
use syn::visit_mut::VisitMut;
use syn::{Ident, ImplItem, ItemImpl, Type, parse_quote, parse2};

pub fn transform_impl_trait(
    item_impl: &ItemImpl,
    consumer_trait_path: &IdentWithTypeArgs,
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

    let mut out_impl = item_impl.clone();

    out_impl.self_ty = Box::new(provider_type.clone());

    let mut provider_trait_path = consumer_trait_path.clone();

    provider_trait_path
        .type_args
        .make_args()
        .insert(0, parse_quote!(#context_type));

    out_impl.trait_ = Some((
        None,
        parse2(provider_trait_path.to_token_stream())?,
        For(Span::call_site()),
    ));

    ReplaceSelfTypeVisitor {
        replaced_type: &context_type,
        skip_assoc_types: &local_assoc_types,
    }
    .visit_item_impl_mut(&mut out_impl);

    ReplaceSelfReceiverVisitor {
        replaced_ident: &context_ident,
        replaced_type: &context_type,
    }
    .visit_item_impl_mut(&mut out_impl);

    ReplaceSelfValueVisitor {
        replaced_ident: &context_ident,
    }
    .visit_item_impl_mut(&mut out_impl);

    Ok(out_impl)
}
