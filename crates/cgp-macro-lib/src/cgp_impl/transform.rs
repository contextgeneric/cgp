use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::token::For;
use syn::{FnArg, Ident, ImplItem, ItemImpl, Type, parse2};

use crate::parse::SimpleType;
use crate::replace_self::{
    replace_self_receiver, replace_self_type_in_generic_args, replace_self_type_in_item_impl,
    replace_self_value_in_block, to_snake_case_ident,
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

    let mut out_impl = item_impl.clone();

    replace_self_type_in_item_impl(&mut out_impl, context_type, &local_assoc_types);

    out_impl.self_ty = Box::new(provider_type.clone());

    let mut provider_trait_path = consumer_trait_path.clone();
    if let Some(generics) = &mut provider_trait_path.generics {
        replace_self_type_in_generic_args(generics, &context_type, &local_assoc_types);
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

    for item in out_impl.items.iter_mut() {
        if let ImplItem::Fn(item_fn) = item
            && let Some(arg) = item_fn.sig.inputs.first_mut()
            && let FnArg::Receiver(receiver) = arg
        {
            *arg = replace_self_receiver(receiver, &context_ident, context_type.to_token_stream());

            replace_self_value_in_block(&mut item_fn.block, &context_ident);
        }
    }

    Ok(out_impl)
}
