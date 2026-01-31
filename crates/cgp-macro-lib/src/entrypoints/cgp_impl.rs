use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::token::{Colon, For};
use syn::{FnArg, Ident, ImplItem, ItemImpl, Type, parse_quote, parse2};

use crate::derive_provider::{
    derive_component_name_from_provider_impl, derive_is_provider_for, derive_provider_struct,
};
use crate::parse::SimpleType;
use crate::replace_self::{
    replace_self_receiver, replace_self_type, replace_self_var, to_snake_case_ident,
};

pub fn cgp_impl(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let spec: ImplProviderSpec = parse2(attr)?;
    let mut item_impl: ItemImpl = parse2(body)?;

    let provider_impl = match &item_impl.trait_ {
        Some((_, path, _)) => {
            let consumer_trait_path = parse2(path.to_token_stream())?;
            let context_type = item_impl.self_ty.as_ref();
            transform_impl_trait(
                &item_impl,
                &consumer_trait_path,
                &spec.provider_type,
                context_type,
            )?
        }
        None => {
            let consumer_trait_path = parse2(item_impl.self_ty.to_token_stream())?;
            let context_type = parse_quote! { __Context__ };

            item_impl
                .generics
                .params
                .insert(0, parse_quote! { __Context__ });

            transform_impl_trait(
                &item_impl,
                &consumer_trait_path,
                &spec.provider_type,
                &context_type,
            )?
        }
    };

    let component_type = match &spec.component_type {
        Some(component_type) => component_type.clone(),
        None => derive_component_name_from_provider_impl(&provider_impl)?,
    };

    let is_provider_for_impl: ItemImpl = derive_is_provider_for(&component_type, &provider_impl)?;

    let provider_struct = if spec.new_struct {
        Some(derive_provider_struct(&provider_impl)?)
    } else {
        None
    };

    Ok(quote! {
        #provider_struct

        #provider_impl

        #is_provider_for_impl
    })
}

pub struct ImplProviderSpec {
    pub new_struct: bool,
    pub provider_type: Type,
    pub component_type: Option<Type>,
}

impl Parse for ImplProviderSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let new_struct = {
            let fork = input.fork();
            let new_ident: Option<Ident> = fork.parse().ok();
            match new_ident {
                Some(new_ident) if new_ident == "new" => {
                    input.advance_to(&fork);
                    true
                }
                _ => false,
            }
        };

        let provider_type = input.parse()?;

        let component_type = if let Some(_colon) = input.parse::<Option<Colon>>()? {
            let component_type: Type = input.parse()?;
            Some(component_type)
        } else {
            None
        };

        Ok(ImplProviderSpec {
            new_struct,
            provider_type,
            component_type,
        })
    }
}

pub fn transform_impl_trait(
    item_impl: &ItemImpl,
    consumer_trait_path: &SimpleType,
    provider_type: &Type,
    context_type: &Type,
) -> syn::Result<ItemImpl> {
    let context_var = if let Ok(ident) = parse2::<Ident>(context_type.to_token_stream()) {
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

    let raw_out_impl = replace_self_type(
        item_impl.to_token_stream(),
        context_type.to_token_stream(),
        &local_assoc_types,
    );

    let mut out_impl: ItemImpl = parse2(raw_out_impl)?;
    out_impl.self_ty = Box::new(provider_type.clone());

    let mut provider_trait_path: SimpleType = consumer_trait_path.clone();

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
            *arg = replace_self_receiver(receiver, &context_var, context_type.to_token_stream());

            let replaced_block = replace_self_var(item_fn.block.to_token_stream(), &context_var);
            item_fn.block = parse2(replaced_block)?;
        }
    }

    Ok(out_impl)
}
