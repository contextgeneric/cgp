use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{FnArg, ItemTrait, Pat, ReturnType, Signature, TraitItem, Type, parse_quote};

pub fn impl_async(item: TokenStream) -> TokenStream {
    match syn::parse2::<ItemTrait>(item.clone()) {
        Ok(mut target_trait) => {
            for trait_item in target_trait.items.iter_mut() {
                if let TraitItem::Fn(trait_fn) = trait_item
                    && trait_fn.sig.asyncness.is_some()
                {
                    let return_type: Type = match &trait_fn.sig.output {
                        ReturnType::Default => {
                            parse_quote!(())
                        }
                        ReturnType::Type(_, return_type) => return_type.as_ref().clone(),
                    };

                    let impl_return: ReturnType = parse_quote! {
                        -> impl ::core::future::Future<Output = #return_type>
                    };

                    trait_fn.sig.output = impl_return;
                    trait_fn.sig.asyncness = None;

                    // A default body becomes the future the method returns, as the body of an
                    // `async fn` is.
                    if let Some(body) = &mut trait_fn.default {
                        let rebinds = move_params_into_future(&mut trait_fn.sig);
                        let stmts = &body.stmts;

                        *body = parse_quote! {
                            {
                                async move {
                                    #(#rebinds)*
                                    #(#stmts)*
                                }
                            }
                        };
                    }
                }
            }

            target_trait.to_token_stream()
        }
        _ => item,
    }
}

/// Rebind each by-value identifier parameter inside the future, so the future owns it and drops it
/// when it completes, as an `async fn` does, rather than when the method returns. A `mut` moves
/// from the parameter to the rebinding, where the body mutates it.
fn move_params_into_future(sig: &mut Signature) -> Vec<TokenStream> {
    let mut rebinds = Vec::new();

    for input in sig.inputs.iter_mut() {
        if let FnArg::Typed(arg) = input
            && let Pat::Ident(pat) = arg.pat.as_mut()
            && pat.by_ref.is_none()
            && pat.subpat.is_none()
        {
            let ident = &pat.ident;
            let mutability = pat.mutability.take();
            rebinds.push(quote! { let #mutability #ident = #ident; });
        }
    }

    rebinds
}
