use cgp_macro_core::functions::merge_generics;
use cgp_macro_core::parse_internal;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ext::IdentExt;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::visit_mut::VisitMut;
use syn::{
    Error, FnArg, GenericParam, Ident, ImplItem, ImplItemFn, ItemTrait, Lifetime, Pat, PatIdent,
    ReturnType, TraitItemFn, Type, Visibility, WherePredicate,
};

use crate::exports::{
    AsyncComputer, Computer, MatchFirstWithValueHandlers, MatchFirstWithValueHandlersMut,
    MatchFirstWithValueHandlersRef, MatchWithValueHandlers, MatchWithValueHandlersMut,
    MatchWithValueHandlersRef,
};
use crate::functions::derive_computer_ident;
use crate::types::cgp_computer::ItemCgpComputer;
use crate::visitors::{ElaborateElidedLifetimes, collect_lifetimes};

/// How a dispatch method takes `self`, with the lifetime a borrowed receiver
/// carries: the one the method names, or the reserved `'__a__` when elided.
pub enum DispatchReceiver {
    Owned,
    Ref(Lifetime),
    Mut(Lifetime),
}

/// One method of a dispatch trait, checked to have a shape the macro can
/// dispatch (a `self` receiver, typed arguments, and lifetime generics only),
/// with every elided lifetime in its signature given a name.
///
/// The names follow the compiler's elision rules, so the generated bound means
/// what the method's signature means: each elided input lifetime is distinct,
/// and an elided output lifetime is the receiver's, or, for a by-value `self`,
/// the single lifetime the arguments use.
pub struct DispatchMethod {
    pub method: TraitItemFn,
    /// The per-variant computer's name: `Compute` plus the method name in
    /// PascalCase.
    pub computer_ident: Ident,
    pub receiver: DispatchReceiver,
    /// The argument types after the receiver, with elided lifetimes named.
    pub arg_types: Vec<Type>,
    /// The return type with elided lifetimes named, or `None` for `()`.
    pub output: Option<Type>,
    /// The lifetimes the elaboration introduced, which the trait and the method
    /// do not declare.
    pub introduced_lifetimes: Vec<Lifetime>,
}

impl DispatchMethod {
    /// Check the method's shape and name its elided lifetimes. Rejects a type or
    /// const generic parameter, a missing or typed receiver, and an untyped
    /// argument, each with a spanned error.
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

        let receiver = match inputs.next() {
            Some(FnArg::Receiver(receiver)) => receiver,
            _ => {
                return Err(Error::new_spanned(
                    sig,
                    "Dispatcher method must have a self argument",
                ));
            }
        };

        // A typed receiver such as `self: Box<Self>` is not the enum or a borrow of
        // it, so no value-handler matcher can take it.
        if receiver.colon_token.is_some() {
            return Err(Error::new_spanned(
                receiver,
                "Dispatcher method receiver must be `self`, `&self`, or `&mut self`",
            ));
        }

        let mut introduced_lifetimes = Vec::new();

        let receiver = match &receiver.reference {
            None => DispatchReceiver::Owned,
            Some((_, lifetime)) => {
                let lifetime = match lifetime {
                    Some(lifetime) => lifetime.clone(),
                    None => {
                        let lifetime: Lifetime = parse_internal!('__a__);
                        introduced_lifetimes.push(lifetime.clone());
                        lifetime
                    }
                };

                if receiver.mutability.is_some() {
                    DispatchReceiver::Mut(lifetime)
                } else {
                    DispatchReceiver::Ref(lifetime)
                }
            }
        };

        let mut elaborate_inputs = ElaborateElidedLifetimes::fresh(1);
        let mut arg_types = Vec::new();

        for input in inputs {
            match input {
                FnArg::Typed(pat_type) => {
                    let mut arg_type = pat_type.ty.as_ref().clone();
                    elaborate_inputs.visit_type_mut(&mut arg_type);
                    arg_types.push(arg_type);
                }
                FnArg::Receiver(_) => {
                    return Err(Error::new_spanned(
                        input,
                        "Dispatcher method arguments must be typed",
                    ));
                }
            }
        }

        introduced_lifetimes.extend(elaborate_inputs.introduced);

        let output = match &sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, output) => {
                let mut output = output.as_ref().clone();

                let output_lifetime = match &receiver {
                    DispatchReceiver::Ref(lifetime) | DispatchReceiver::Mut(lifetime) => {
                        Some(lifetime.clone())
                    }
                    DispatchReceiver::Owned => {
                        let input_lifetimes = collect_lifetimes(&arg_types);
                        if input_lifetimes.len() == 1 {
                            input_lifetimes.into_iter().next()
                        } else {
                            // The compiler rejects the trait itself here, since the
                            // elided output lifetime has no lifetime to take.
                            None
                        }
                    }
                };

                if let Some(lifetime) = output_lifetime {
                    ElaborateElidedLifetimes::fixed(lifetime).visit_type_mut(&mut output);
                }

                Some(output)
            }
        };

        Ok(Self {
            method: method.clone(),
            computer_ident: derive_computer_ident(&sig.ident),
            receiver,
            arg_types,
            output,
            introduced_lifetimes,
        })
    }

    /// The type the matcher receives as the payload: the enum itself, or a
    /// borrow of it.
    fn context_type(&self) -> TokenStream {
        match &self.receiver {
            DispatchReceiver::Owned => quote! { __Variants__ },
            DispatchReceiver::Ref(lifetime) => quote! { & #lifetime __Variants__ },
            DispatchReceiver::Mut(lifetime) => quote! { & #lifetime mut __Variants__ },
        }
    }

    fn output_type(&self) -> TokenStream {
        match &self.output {
            Some(output) => quote! { #output },
            None => quote! { () },
        }
    }

    /// The blanket impl's method, which runs the matcher selected for this
    /// method's shape, and the `where` predicate bounding that matcher.
    pub fn to_blanket_impl_item(&self) -> syn::Result<(ImplItem, WherePredicate)> {
        let mut signature = self.method.sig.clone();
        let computer_ident = &self.computer_ident;

        // Rebind the arguments positionally, so the body can forward them
        // whatever patterns the trait declares them with.
        let mut arg_idents = Punctuated::<Ident, Comma>::new();

        for (i, arg) in signature.inputs.iter_mut().skip(1).enumerate() {
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
            }
        }

        let has_args = !self.arg_types.is_empty();

        let matcher = match (&self.receiver, has_args) {
            (DispatchReceiver::Owned, false) => quote! { #MatchWithValueHandlers },
            (DispatchReceiver::Owned, true) => quote! { #MatchFirstWithValueHandlers },
            (DispatchReceiver::Ref(_), false) => quote! { #MatchWithValueHandlersRef },
            (DispatchReceiver::Ref(_), true) => quote! { #MatchFirstWithValueHandlersRef },
            (DispatchReceiver::Mut(_), false) => quote! { #MatchWithValueHandlersMut },
            (DispatchReceiver::Mut(_), true) => quote! { #MatchFirstWithValueHandlersMut },
        };

        let context_type = self.context_type();
        let output_type = self.output_type();
        let arg_types = &self.arg_types;

        let input_type = if has_args {
            quote! { (#context_type, (#(#arg_types),*)) }
        } else {
            quote! { #context_type }
        };

        // Quantify the method's own lifetimes and the ones the elaboration named,
        // but never the trait's, which the impl already declares.
        let quantified: Vec<&Lifetime> = signature
            .generics
            .lifetimes()
            .map(|param| &param.lifetime)
            .chain(self.introduced_lifetimes.iter())
            .collect();

        let hrtb = if quantified.is_empty() {
            TokenStream::new()
        } else {
            quote! { for< #(#quantified),* > }
        };

        let args = if has_args {
            quote! { (self, (#arg_idents)) }
        } else {
            quote! { self }
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
                    // arguments are inferred, which avoids naming the lifetimes only
                    // the bound's `for<..>` declares.
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
        let method_ident = &self.method.sig.ident;
        let async_token = &self.method.sig.asyncness;
        let trait_ident = &item_trait.ident;
        let type_generics = item_trait.generics.split_for_impl().1;

        let mut generics = merge_generics(&item_trait.generics, &self.method.sig.generics);
        // Insert the payload parameter, then the introduced lifetimes, as leading
        // generics. Position 0 is safe with a lifetime present because
        // `syn::Generics::to_tokens` emits lifetimes first. See
        // cgp-knowledge-base/cgp/implementation/README.md, "Generic-parameter
        // insertion and lifetime ordering".
        generics.params.insert(
            0,
            parse_internal!(__Variants__: #trait_ident #type_generics),
        );

        for lifetime in self.introduced_lifetimes.iter().rev() {
            generics.params.insert(0, parse_internal!(#lifetime));
        }

        let context_type = self.context_type();
        let arg_types = &self.arg_types;

        let arg_idents: Punctuated<Ident, Comma> = self
            .method
            .sig
            .inputs
            .iter()
            .skip(1)
            .enumerate()
            .map(|(i, arg)| Ident::new(&format!("arg_{i}"), arg.span()))
            .collect();

        let arg_params = if arg_idents.is_empty() {
            TokenStream::new()
        } else {
            quote! {
                (#arg_idents): (#(#arg_types),*)
            }
        };

        let return_type = match &self.output {
            Some(output) => quote! { -> #output },
            None => TokenStream::new(),
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
