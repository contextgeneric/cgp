use std::collections::BTreeSet;

use cgp_macro_core::functions::{merge_generics, to_camel_case_str};
use cgp_macro_core::parse_internal;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::ext::IdentExt;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    Error, FnArg, GenericParam, Ident, ImplItem, ImplItemFn, ItemTrait, Lifetime, Pat, PatIdent,
    ReturnType, TraitItemFn, Type, Visibility, WherePredicate,
};

use crate::exports::{
    AsyncComputer, Computer, MatchFirstWithValueHandlers, MatchFirstWithValueHandlersMut,
    MatchFirstWithValueHandlersRef, MatchWithValueHandlers, MatchWithValueHandlersMut,
    MatchWithValueHandlersRef,
};
use crate::types::cgp_computer::ItemCgpComputer;

/// One method of a dispatch trait, checked to have a shape the macro can
/// dispatch: a `self` receiver, typed arguments, and lifetime generics only.
pub struct DispatchMethod {
    pub method: TraitItemFn,
    /// The per-variant computer's name: `Compute` plus the method name in
    /// PascalCase.
    pub computer_ident: Ident,
}

impl DispatchMethod {
    pub fn new(method: &TraitItemFn) -> syn::Result<Self> {
        let sig = &method.sig;

        for generic_param in sig.generics.params.iter() {
            if !matches!(generic_param, GenericParam::Lifetime(_)) {
                return Err(Error::new_spanned(
                    generic_param,
                    "Dispatch trait methods cannot contain non-lifetime generic parameters due to the lack of quantified constraints in Rust",
                ));
            }
        }

        let mut inputs = sig.inputs.iter();

        if !matches!(inputs.next(), Some(FnArg::Receiver(_))) {
            return Err(Error::new_spanned(
                sig,
                "Dispatcher method must have a self argument",
            ));
        }

        for input in inputs {
            if !matches!(input, FnArg::Typed(_)) {
                return Err(Error::new_spanned(
                    input,
                    "Dispatcher method arguments must be typed",
                ));
            }
        }

        Ok(Self {
            method: method.clone(),
            computer_ident: derive_computer_ident(&sig.ident),
        })
    }

    /// The blanket impl's method, which runs the matcher selected for this
    /// method's shape, and the `where` predicate bounding that matcher.
    pub fn to_blanket_impl_item(&self) -> syn::Result<(ImplItem, WherePredicate)> {
        let mut signature = self.method.sig.clone();
        let computer_ident = &self.computer_ident;

        let extra_life: Lifetime = parse_internal!('__a__);
        let mut hrtbs: BTreeSet<Ident> = BTreeSet::new();

        let mut args = signature.inputs.iter_mut();

        let receiver = match args.next() {
            Some(FnArg::Receiver(receiver)) => receiver.clone(),
            _ => {
                return Err(Error::new_spanned(
                    &self.method.sig,
                    "Dispatcher method must have a self argument",
                ));
            }
        };

        let mut arg_idents = Punctuated::<Ident, Comma>::new();
        let mut arg_types = Punctuated::<Type, Comma>::new();

        for (i, arg) in args.enumerate() {
            if let FnArg::Typed(pat_type) = arg {
                let arg_ident = Ident::new(&format!("arg_{i}"), pat_type.span());
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
            }
        }

        let output_type: Type = match &signature.output {
            ReturnType::Default => parse_internal!(()),
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
                output
            }
        };

        let (context_type, matcher) = if let Some((_, life)) = &receiver.reference {
            let life = life.as_ref().unwrap_or_else(|| {
                hrtbs.insert(extra_life.ident.clone());
                &extra_life
            });

            let mutability = &receiver.mutability;
            let context_type = quote! { & #life #mutability __Variants__ };
            let matcher = match (mutability.is_some(), arg_types.is_empty()) {
                (true, true) => quote! { #MatchWithValueHandlersMut },
                (true, false) => quote! { #MatchFirstWithValueHandlersMut },
                (false, true) => quote! { #MatchWithValueHandlersRef },
                (false, false) => quote! { #MatchFirstWithValueHandlersRef },
            };

            (context_type, matcher)
        } else {
            let matcher = if arg_types.is_empty() {
                quote! { #MatchWithValueHandlers }
            } else {
                quote! { #MatchFirstWithValueHandlers }
            };

            (quote! { __Variants__ }, matcher)
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

        let args = if arg_idents.is_empty() {
            quote! { self }
        } else {
            quote! { (self, (#arg_idents)) }
        };

        let (predicate, method_body): (WherePredicate, TokenStream) =
            if signature.asyncness.is_some() {
                (
                    parse_internal! {
                        #matcher<#computer_ident>: #hrtb
                            #AsyncComputer<(), (), #input_type, Output = #output_type>
                    },
                    // Name the provider trait so the call stays unambiguous when the
                    // consumer trait `CanComputeAsync` is also in scope. The `_`
                    // arguments are inferred, which avoids naming the HRTB-only
                    // `'__a__` lifetime.
                    quote! {
                        <#matcher<#computer_ident> as #AsyncComputer<_, _, _>>::compute_async(
                            &(),
                            ::core::marker::PhantomData::<()>,
                            #args,
                        ).await
                    },
                )
            } else {
                (
                    parse_internal! {
                        #matcher<#computer_ident>: #hrtb
                            #Computer<(), (), #input_type, Output = #output_type>
                    },
                    // As above, qualified so that an imported `CanCompute` does not
                    // make the call ambiguous.
                    quote! {
                        <#matcher<#computer_ident> as #Computer<_, _, _>>::compute(
                            &(),
                            ::core::marker::PhantomData::<()>,
                            #args,
                        )
                    },
                )
            };

        let impl_item = ImplItem::Fn(ImplItemFn {
            attrs: Default::default(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig: signature,
            block: parse_internal!({ #method_body }),
        });

        Ok((impl_item, predicate))
    }

    /// The `#[cgp_computer]` input for this method's per-variant computer: a
    /// private helper function, generic over any payload implementing the
    /// trait, that calls the method on the payload.
    pub fn to_computer(&self, item_trait: &ItemTrait) -> syn::Result<ItemCgpComputer> {
        let mut signature = self.method.sig.clone();
        let method_ident = &self.method.sig.ident;
        let async_token = signature.asyncness;
        let trait_ident = &item_trait.ident;
        let type_generics = item_trait.generics.split_for_impl().1;

        let mut generics = merge_generics(&item_trait.generics, &signature.generics);
        // Insert the payload parameter as the leading generic. Position 0 is safe
        // with a lifetime present because `syn::Generics::to_tokens` emits
        // lifetimes first. See cgp-knowledge-base/cgp/implementation/README.md,
        // "Generic-parameter insertion and lifetime ordering".
        generics.params.insert(
            0,
            parse_internal!(__Variants__: #trait_ident #type_generics),
        );

        let mut args = signature.inputs.iter_mut();

        let receiver = match args.next() {
            Some(FnArg::Receiver(receiver)) => receiver.clone(),
            _ => {
                return Err(Error::new_spanned(
                    &self.method.sig,
                    "Dispatcher method must have a self argument",
                ));
            }
        };

        let extra_life: Lifetime = parse_internal!('__a__);
        let mut use_extra_life = false;

        let context_type = match (&receiver.reference, &receiver.mutability) {
            (Some((_, life)), Some(_)) => {
                let life = life.as_ref().unwrap_or_else(|| {
                    use_extra_life = true;
                    &extra_life
                });

                quote! { &#life mut __Variants__ }
            }
            (Some((_, life)), None) => {
                let life = life.as_ref().unwrap_or_else(|| {
                    use_extra_life = true;
                    &extra_life
                });

                quote! { & #life __Variants__ }
            }
            _ => quote! { __Variants__ },
        };

        let mut arg_idents = Punctuated::<Ident, Comma>::new();
        let mut arg_types = Punctuated::<Type, Comma>::new();

        for (i, arg) in args.enumerate() {
            if let FnArg::Typed(pat_type) = arg {
                arg_idents.push(Ident::new(&format!("arg_{i}"), pat_type.span()));

                let arg_type = pat_type.ty.as_mut();
                if let Type::Reference(arg_type) = arg_type
                    && arg_type.lifetime.is_none()
                {
                    use_extra_life = true;
                    arg_type.lifetime = Some(extra_life.clone());
                }

                arg_types.push(arg_type.clone());
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
            // The same lifetime-first emission makes inserting `'__a__` at
            // position 0 safe; see the insertion above.
            generics.params.insert(0, parse_internal!(#extra_life));
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

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        // The helper function takes a reserved name rather than the method's own,
        // so it cannot collide with an item of the same name already in the
        // caller's module.
        let helper_ident = Ident::new(
            &format!("__compute_{}__", method_ident.unraw()),
            method_ident.span(),
        );

        Ok(ItemCgpComputer {
            ident: Some(self.computer_ident.clone()),
            item_fn: parse_internal! {
                #async_token fn #helper_ident #impl_generics (
                    __Variants__: #context_type,
                    #arg_params
                ) #return_type
                #where_clause
                {
                    __Variants__. #method_ident ( #arg_idents ) #dot_await
                }
            },
        })
    }
}

/// The per-variant computer's name, `Compute` plus the method name in PascalCase.
/// The method name is unrawed first, so `r#type` yields `ComputeType` rather than
/// an invalid identifier.
fn derive_computer_ident(method_ident: &Ident) -> Ident {
    Ident::new(
        &format!(
            "Compute{}",
            to_camel_case_str(&method_ident.unraw().to_string())
        ),
        method_ident.span(),
    )
}
