use proc_macro2::{Group, Span, TokenStream, TokenTree};
use quote::{ToTokens, format_ident, quote};
use syn::token::For;
use syn::{Ident, ImplItem, ItemImpl, Type, parse2};

use crate::derive_component::{replace_self_receiver, replace_self_type, to_snake_case_ident};
use crate::parse::SimpleType;

pub fn transform_impl_trait(
    item_impl: &ItemImpl,
    provider_trait_ident: &Ident,
    provider_type: &Type,
) -> syn::Result<ItemImpl> {
    let context_type = item_impl.self_ty.as_ref();

    let (context_ident, use_refl) =
        if let Some(ident) = parse2::<Ident>(context_type.to_token_stream()).ok() {
            (ident, false)
        } else {
            (Ident::new("__Context__", Span::call_site()), true)
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
        &context_ident,
        &local_assoc_types,
    );

    let mut out_impl: ItemImpl = parse2(raw_out_impl)?;
    out_impl.self_ty = Box::new(provider_type.clone());

    let source_trait_path = &item_impl.trait_.as_ref().unwrap().1;
    let mut provider_trait_path: SimpleType = parse2(source_trait_path.to_token_stream())?;
    provider_trait_path.name = provider_trait_ident.clone();

    match &mut provider_trait_path.generics {
        Some(generics) => {
            generics
                .args
                .insert(0, parse2(context_ident.to_token_stream())?);
        }
        None => {
            provider_trait_path.generics = Some(parse2(quote! { < #context_ident > })?);
        }
    }

    out_impl.trait_ = Some((
        None,
        parse2(provider_trait_path.to_token_stream())?,
        For(Span::call_site()),
    ));

    if use_refl {
        let where_clause = out_impl.generics.make_where_clause();
        where_clause.predicates.push(parse2(quote! {
            #context_ident: Refl<Type = #context_type>
        })?);
    }

    // let mut provider_trait_spec = item_impl.trait_.clone().unwrap();
    // let provider_trait_path = &mut provider_trait_spec.1;
    // let segment = provider_trait_path.segments.last_mut().unwrap();

    // match &mut segment.arguments {
    //     PathArguments::None => {
    //         segment.arguments = PathArguments::AngleBracketed(parse2(quote! { < #context_ident > })?);
    //     }
    //     PathArguments::AngleBracketed(args) => {
    //         args.args.insert(0, parse2(quote! { #context_ident })?);
    //     }
    //     _ => {
    //         return Err(Error::new(segment.span(), "trait path must end with angle bracket generic arguments"))
    //     }
    // }

    // let provider_trait_path = item_impl.trait_.unwrap().1.clone();
    // out_impl.trait_ = Some((None, provider_trait_path, For(Span::call_site())));

    // let context_param: GenericParam = parse2(context_type.to_token_stream())?;
    // out_impl.generics.params.insert(0, context_param);

    for item in out_impl.items.iter_mut() {
        if let ImplItem::Fn(item_fn) = item {
            replace_self_receiver(&mut item_fn.sig, &context_ident);
            let context_var = to_snake_case_ident(&context_ident);
            let replaced_block = replace_self_var(item_fn.block.to_token_stream(), &context_var);
            item_fn.block = parse2(replaced_block)?;
        }
    }

    Ok(out_impl)
}

fn replace_self_var(stream: TokenStream, replaced_ident: &Ident) -> TokenStream {
    let self_ident = format_ident!("self");

    let mut result_stream: Vec<TokenTree> = Vec::new();

    let mut token_iter = stream.into_iter();

    while let Some(tree) = token_iter.next() {
        match tree {
            TokenTree::Ident(ident) => {
                if ident == self_ident {
                    result_stream.push(TokenTree::Ident(replaced_ident.clone()));
                } else {
                    result_stream.push(TokenTree::Ident(ident));
                }
            }
            TokenTree::Group(group) => {
                let replaced_stream = replace_self_var(group.stream(), replaced_ident);
                let replaced_group = Group::new(group.delimiter(), replaced_stream);

                result_stream.push(TokenTree::Group(replaced_group));
            }
            TokenTree::Punct(punct) => {
                result_stream.push(TokenTree::Punct(punct));
            }
            TokenTree::Literal(lit) => result_stream.push(TokenTree::Literal(lit)),
        }
    }

    result_stream.into_iter().collect()
}
