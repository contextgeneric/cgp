use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Expr, Ident, ItemFn, ItemImpl, Type};

use crate::extra_macro_core::exports::{
    AsyncComputer, AsyncComputerComponent, AsyncComputerRefComponent, Computer, ComputerComponent,
    ComputerRefComponent, HandlerComponent, HandlerRefComponent, PromoteAsyncComputer,
    PromoteComputer, PromoteHandler, PromoteTryComputer, TryComputerComponent,
    TryComputerRefComponent,
};
use crate::extra_macro_core::types::handler_fn::EvaluatedHandlerFn;
use crate::macro_core::functions::override_item_span;
use crate::macro_core::parse_internal;
use crate::macro_core::traits::ToTypeParamBounds;
use crate::macro_core::types::cgp_provider::{ItemCgpProvider, ProviderArgs};
use crate::macro_core::types::delegate_component::DelegateTable;
use crate::macro_core::types::implicits::ImplicitArgFields;
use crate::macro_core::types::keyword::Keyword;

/// The read `#[cgp_computer]` input: the resolved provider name, the function,
/// its explicit inputs rebound as `arg_0, arg_1, …`, the arguments of the call
/// back to the function (context-field reads mixed with those inputs), its
/// output type, and the two facts that choose the expansion.
pub struct PreprocessedCgpComputer {
    pub provider_ident: Ident,
    pub item_fn: ItemFn,
    pub input_idents: Punctuated<Ident, Comma>,
    pub input_types: Punctuated<Type, Comma>,
    /// Arguments of the call to the user's function, in parameter order.
    pub call_args: Punctuated<Expr, Comma>,
    /// `#[implicit]` and `#[field]` parameters, read from the context.
    pub context_fields: ImplicitArgFields,
    pub output: Type,
    /// An `async` function implements `AsyncComputer` rather than `Computer`.
    pub is_async: bool,
    /// A `Result<T, E>` return selects the fallible promotion bundle.
    pub is_fallible: bool,
}

impl PreprocessedCgpComputer {
    /// Evaluate into the shared handler-function IR: a `Computer` (or
    /// `AsyncComputer`) impl that calls the function, and a table routing the
    /// rest of the handler family to the promotion bundle the two facts select.
    pub fn eval(&self) -> syn::Result<EvaluatedHandlerFn> {
        let provider_ident = &self.provider_ident;
        let fn_ident = &self.item_fn.sig.ident;
        let input_idents = &self.input_idents;
        let input_types = &self.input_types;
        let call_args = &self.call_args;
        let output = &self.output;

        // A context-field read names the receiver `context`. With no such read the
        // receiver stays `_context`, unused.
        let context_ident = if self.context_fields.fields.is_empty() {
            Ident::new("_context", proc_macro2::Span::call_site())
        } else {
            Ident::new("context", fn_ident.span())
        };

        // The function's generics carry over, and the reserved context and code
        // parameters are appended after them.
        let mut generics = self.item_fn.sig.generics.clone();
        generics.params.push(parse_internal!(__Context__));
        generics.params.push(parse_internal!(__Code__));

        if !self.context_fields.fields.is_empty() {
            let bounds = self.context_fields.to_type_param_bounds()?;
            generics
                .make_where_clause()
                .predicates
                .push(parse_internal!(__Context__: #bounds));
        }

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let (item_impl, component_type, delegate_table): (ItemImpl, Type, DelegateTable) =
            if self.is_async {
                let item_impl = parse_internal! {
                    impl #impl_generics
                        #AsyncComputer<__Context__, __Code__, ( #input_types )>
                        for #provider_ident
                    #where_clause
                    {
                        type Output = #output;

                        async fn compute_async(
                            #context_ident: &__Context__,
                            _code: ::core::marker::PhantomData<__Code__>,
                            ( #input_idents ): ( #input_types ),
                        ) -> Self::Output {
                            #fn_ident( #call_args ).await
                        }
                    }
                };

                // The synchronous members of the family cannot be derived from an
                // async base, so only the async ones are promoted.
                let delegate_table = if self.is_fallible {
                    parse_internal! {
                        #provider_ident {
                            [
                                #AsyncComputerRefComponent,
                                #HandlerComponent,
                                #HandlerRefComponent,
                            ] ->
                                #PromoteHandler<Self>,
                        }
                    }
                } else {
                    parse_internal! {
                        #provider_ident {
                            [
                                #AsyncComputerRefComponent,
                                #HandlerComponent,
                                #HandlerRefComponent,
                            ] ->
                                #PromoteAsyncComputer<Self>,
                        }
                    }
                };

                (
                    item_impl,
                    parse_internal!(#AsyncComputerComponent),
                    delegate_table,
                )
            } else {
                let item_impl = parse_internal! {
                    impl #impl_generics
                        #Computer<__Context__, __Code__, ( #input_types )>
                        for #provider_ident
                    #where_clause
                    {
                        type Output = #output;

                        fn compute(
                            #context_ident: &__Context__,
                            _code: ::core::marker::PhantomData<__Code__>,
                            ( #input_idents ): ( #input_types ),
                        ) -> Self::Output {
                            #fn_ident( #call_args )
                        }
                    }
                };

                let delegate_table = if self.is_fallible {
                    parse_internal! {
                        #provider_ident {
                            [
                                #ComputerRefComponent,
                                #TryComputerComponent,
                                #TryComputerRefComponent,
                                #AsyncComputerComponent,
                                #AsyncComputerRefComponent,
                                #HandlerComponent,
                                #HandlerRefComponent,
                            ] ->
                                #PromoteTryComputer<Self>,
                        }
                    }
                } else {
                    parse_internal! {
                        #provider_ident {
                            [
                                #ComputerRefComponent,
                                #TryComputerComponent,
                                #TryComputerRefComponent,
                                #AsyncComputerComponent,
                                #AsyncComputerRefComponent,
                                #HandlerComponent,
                                #HandlerRefComponent,
                            ] ->
                                #PromoteComputer<Self>,
                        }
                    }
                };

                (
                    item_impl,
                    parse_internal!(#ComputerComponent),
                    delegate_table,
                )
            };

        // Re-span the impl's boundary tokens onto the function name, so an error on
        // the generated impl (a conflict with another provider of the same name,
        // say) points at the function rather than the whole attribute. See
        // cgp-knowledge-base/cgp/implementation/README.md, "Spans".
        let item_impl = override_item_span(fn_ident.span(), &item_impl)?;

        let provider = ItemCgpProvider {
            args: ProviderArgs {
                new: Some(Keyword::default()),
                component_type: Some(component_type),
            },
            item_impl,
        };

        Ok(EvaluatedHandlerFn {
            item_fn: self.item_fn.clone(),
            provider,
            delegate_table,
        })
    }
}
