use std::collections::BTreeSet;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    FnArg, GenericParam, Ident, ImplItem, ImplItemFn, ItemTrait, Lifetime, Pat, PatIdent,
    ReturnType, TraitItemFn, Type, Visibility, parse2,
};

use crate::utils::to_camel_case_str;

pub fn cgp_auto_dispatch(_attr: TokenStream, mut out: TokenStream) -> syn::Result<TokenStream> {
    let item_trait: ItemTrait = parse2(out.clone())?;

    let blanket_impl = derive_blanket_impl(&item_trait)?;
    out.extend(blanket_impl);

    for item in item_trait.items.iter() {
        match item {
            syn::TraitItem::Fn(fn_item) => {
                let method_computer = derive_method_computer(&item_trait, fn_item)?;
                out.extend(method_computer);
            }
            _ => {
                return Err(syn::Error::new(
                    item.span(),
                    "Only function items are allowed in a dispatch trait",
                ));
            }
        }
    }

    Ok(out)
}

fn derive_blanket_impl(item_trait: &ItemTrait) -> syn::Result<TokenStream> {
    let trait_ident = &item_trait.ident;
    let context_ident = quote! { __Variants__ };

    let mut generics = item_trait.generics.clone();
    generics
        .params
        .insert(0, parse2(quote! { #context_ident })?);

    let where_clause = generics.make_where_clause();

    let extra_life: Lifetime = parse2(quote! { '__a__ })?;

    let mut impl_items: Vec<ImplItem> = Vec::new();

    for trait_item in item_trait.items.iter() {
        let method = if let syn::TraitItem::Fn(method) = trait_item {
            method
        } else {
            return Err(syn::Error::new(
                trait_item.span(),
                "Only function items are allowed in a dispatch trait",
            ));
        };

        let mut signature = method.sig.clone();
        let method_ident = &signature.ident;
        let mut hrtbs: BTreeSet<Ident> = BTreeSet::new();

        let computer_ident = Ident::new(
            &format!("Compute{}", to_camel_case_str(&method_ident.to_string())),
            method_ident.span(),
        );

        for generic_param in signature.generics.params.iter() {
            match generic_param {
                GenericParam::Lifetime(_) => {}
                _ => {
                    return Err(syn::Error::new(
                        generic_param.span(),
                        "Dispatch trait methods cannot contain non-lifetime generic parameters due to the lack of quantified constraints in Rust",
                    ));
                }
            }
        }

        let mut args = signature.inputs.iter_mut();

        let receiver = if let Some(FnArg::Receiver(receiver)) = args.next() {
            receiver
        } else {
            return Err(syn::Error::new(
                signature.span(),
                "Dispatcher method must have a self argument",
            ));
        };

        let mut arg_idents = Punctuated::<_, Comma>::new();
        let mut arg_types = Punctuated::<_, Comma>::new();

        for (i, arg) in args.enumerate() {
            if let FnArg::Typed(pat_type) = arg {
                let arg_ident = Ident::new(&format!("arg_{}", i), pat_type.span());
                arg_idents.push(arg_ident.clone());
                *pat_type.pat = Pat::Ident(PatIdent {
                    ident: arg_ident,
                    attrs: Default::default(),
                    by_ref: Default::default(),
                    mutability: Default::default(),
                    subpat: Default::default(),
                });

                let mut arg_type = pat_type.ty.as_ref().clone();
                if let Type::Reference(arg_type) = &mut arg_type {
                    match &arg_type.lifetime {
                        Some(lifetime) => {
                            hrtbs.insert(lifetime.ident.clone());
                        }
                        None => {
                            hrtbs.insert(extra_life.ident.clone());
                            arg_type.lifetime = Some(extra_life.clone());
                        }
                    }
                }

                arg_types.push(arg_type);
            } else {
                return Err(syn::Error::new(
                    arg.span(),
                    "Dispatcher method arguments must be typed",
                ));
            }
        }

        let output_type = match &signature.output {
            ReturnType::Default => {
                quote! { () }
            }
            ReturnType::Type(_, output) => {
                let mut output = output.as_ref().clone();
                if let Type::Reference(output_type) = &mut output {
                    match &output_type.lifetime {
                        Some(lifetime) => {
                            hrtbs.insert(lifetime.ident.clone());
                        }
                        None => {
                            hrtbs.insert(extra_life.ident.clone());
                            output_type.lifetime = Some(extra_life.clone());
                        }
                    }
                }
                quote! { #output }
            }
        };

        let (context_type, matcher) = if let Some((_, life)) = &receiver.reference {
            let life = life.as_ref().unwrap_or_else(|| {
                hrtbs.insert(extra_life.ident.clone());
                &extra_life
            });

            let mutability = &receiver.mutability;
            let context_type = quote! { & #life #mutability #context_ident };
            let matcher = if mutability.is_some() {
                if arg_types.is_empty() {
                    quote! { MatchWithValueHandlersMut }
                } else {
                    quote! { MatchFirstWithValueHandlersMut }
                }
            } else if arg_types.is_empty() {
                quote! { MatchWithValueHandlersRef }
            } else {
                quote! { MatchFirstWithValueHandlersRef }
            };

            (context_type, matcher)
        } else {
            let context_type = quote! { #context_ident  };
            let matcher = if arg_types.is_empty() {
                quote! { MatchWithValueHandlers }
            } else {
                quote! { MatchFirstWithValueHandlers }
            };

            (context_type, matcher)
        };

        let mut hrtb = TokenStream::new();

        for ident in hrtbs {
            if ident != "static" {
                let lifetime = Lifetime {
                    apostrophe: Span::call_site(),
                    ident,
                };
                hrtb = quote! { for<#lifetime> }
            }
        }

        let input_type = if arg_types.is_empty() {
            quote! { #context_type }
        } else {
            quote! { (#context_type, (#arg_types)) }
        };

        if signature.asyncness.is_some() {
            where_clause.predicates.push(parse2(quote! {
                #matcher<#computer_ident>: #hrtb
                    AsyncComputer<(), (), #input_type, Output = #output_type>
            })?);
        } else {
            where_clause.predicates.push(parse2(quote! {
                #matcher<#computer_ident>: #hrtb
                    Computer<(), (), #input_type, Output = #output_type>
            })?);
        }

        let args = if arg_idents.is_empty() {
            quote! { self }
        } else {
            quote! { (self, (#arg_idents)) }
        };

        let method_body = if signature.asyncness.is_some() {
            quote! {
                #matcher::<#computer_ident>::compute_async(
                    &(),
                    ::core::marker::PhantomData::<()>,
                    #args,
                ).await
            }
        } else {
            quote! {
                #matcher::<#computer_ident>::compute(
                    &(),
                    ::core::marker::PhantomData::<()>,
                    #args,
                )
            }
        };

        let impl_item = ImplItem::Fn(ImplItemFn {
            attrs: Default::default(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig: signature,
            block: parse2(quote! {
                { #method_body }
            })?,
        });

        impl_items.push(impl_item);
    }

    where_clause.predicates.push(parse2(quote! {
        #context_ident: HasExtractor
    })?);

    let ty_generics = item_trait.generics.split_for_impl().1;
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let item_impl = quote! {
        impl #impl_generics #trait_ident #ty_generics for #context_ident
            #where_clause
        {
            #(#impl_items)*
        }
    };

    Ok(item_impl)
}

fn derive_method_computer(
    item_trait: &ItemTrait,
    method: &TraitItemFn,
) -> syn::Result<TokenStream> {
    let mut signature = method.sig.clone();
    let method_ident = &signature.ident;
    let async_token = signature.asyncness;

    let context_ident = quote! { __Variants__ };

    let mut generics = {
        let mut generics = item_trait.generics.clone();

        generics
            .params
            .extend(signature.generics.params.iter().cloned());

        if let Some(method_where_clause) = &signature.generics.where_clause {
            generics
                .make_where_clause()
                .predicates
                .extend(method_where_clause.predicates.iter().cloned());
        }

        let trait_ident = &item_trait.ident;

        let type_generics = item_trait.generics.split_for_impl().1;

        generics.params.insert(
            0,
            parse2(quote! {
                #context_ident: #trait_ident #type_generics
            })?,
        );

        generics
    };

    let mut args = signature.inputs.iter_mut();

    let receiver = if let Some(FnArg::Receiver(receiver)) = args.next() {
        receiver
    } else {
        return Err(syn::Error::new(
            signature.span(),
            "Dispatcher method must have a self argument",
        ));
    };

    let extra_life: Lifetime = parse2(quote! { '__a__ })?;
    let mut use_extra_life = false;

    let context_type = match (&receiver.reference, &receiver.mutability) {
        (Some((_, life)), Some(_)) => {
            let life = life.as_ref().unwrap_or_else(|| {
                use_extra_life = true;
                &extra_life
            });

            quote! { &#life mut #context_ident }
        }
        (Some((_, life)), None) => {
            let life = life.as_ref().unwrap_or_else(|| {
                use_extra_life = true;
                &extra_life
            });

            quote! { & #life #context_ident }
        }
        _ => quote! { #context_ident },
    };

    let mut arg_idents = Punctuated::<_, Comma>::new();
    let mut arg_types = Punctuated::<_, Comma>::new();

    for (i, arg) in args.enumerate() {
        if let FnArg::Typed(pat_type) = arg {
            arg_idents.push(Ident::new(&format!("arg_{}", i), pat_type.span()));

            let arg_type = pat_type.ty.as_mut();
            if let Type::Reference(arg_type) = arg_type
                && arg_type.lifetime.is_none()
            {
                use_extra_life = true;
                arg_type.lifetime = Some(extra_life.clone());
            }

            arg_types.push(arg_type);
        } else {
            return Err(syn::Error::new(
                arg.span(),
                "Dispatcher method arguments must be typed",
            ));
        }
    }

    let return_type = &mut signature.output;

    if let ReturnType::Type(_, return_type) = return_type
        && let Type::Reference(return_type) = return_type.as_mut()
        && return_type.lifetime.is_none()
    {
        use_extra_life = true;
        return_type.lifetime = Some(extra_life.clone());
    }

    if use_extra_life {
        generics.params.insert(0, parse2(quote! { #extra_life })?);
    }

    let arg_params = if arg_idents.is_empty() {
        TokenStream::new()
    } else {
        quote! {
            (#arg_idents): (#arg_types)
        }
    };

    let dot_await = if async_token.is_some() {
        quote! { .await }
    } else {
        TokenStream::new()
    };

    let computer_ident = Ident::new(
        &format!("Compute{}", to_camel_case_str(&method_ident.to_string())),
        method_ident.span(),
    );

    let method_generics = {
        let method_generics = method
            .sig
            .generics
            .params
            .iter()
            .filter(|param| !matches!(param, syn::GenericParam::Lifetime(_)))
            .collect::<Punctuated<_, Comma>>();

        if method_generics.is_empty() {
            TokenStream::new()
        } else {
            quote! { ::< #method_generics > }
        }
    };

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #[cgp_computer( #computer_ident )]
        #async_token fn #method_ident #impl_generics (
            #context_ident: #context_type,
            #arg_params
        ) #return_type
        #where_clause
        {
            #context_ident. #method_ident #method_generics ( #arg_idents ) #dot_await
        }
    })
}
