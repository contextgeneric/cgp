use cgp_macro_core::functions::override_item_span;
use cgp_macro_core::parse_internal;
use cgp_macro_core::types::cgp_provider::{ItemCgpProvider, ProviderArgs};
use cgp_macro_core::types::delegate_component::DelegateTable;
use cgp_macro_core::types::keyword::Keyword;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Ident, ItemFn, ItemImpl, Type};

use crate::exports::{
    AsyncComputer, AsyncComputerComponent, AsyncComputerRefComponent, Computer, ComputerComponent,
    ComputerRefComponent, HandlerComponent, HandlerRefComponent, PromoteAsyncComputer,
    PromoteComputer, PromoteHandler, PromoteTryComputer, TryComputerComponent,
    TryComputerRefComponent,
};
use crate::types::handler_fn::EvaluatedHandlerFn;

/// The read `#[cgp_computer]` input: the resolved provider name, the function,
/// its inputs rebound as `arg_0, arg_1, …`, its output type, and the two facts
/// that choose the expansion.
pub struct PreprocessedCgpComputer {
    pub provider_ident: Ident,
    pub item_fn: ItemFn,
    pub input_idents: Punctuated<Ident, Comma>,
    pub input_types: Punctuated<Type, Comma>,
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
        let output = &self.output;

        // The function's generics carry over, and the reserved context and code
        // parameters are appended after them.
        let mut generics = self.item_fn.sig.generics.clone();
        generics.params.push(parse_internal!(__Context__));
        generics.params.push(parse_internal!(__Code__));

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
                            _context: &__Context__,
                            _code: ::core::marker::PhantomData<__Code__>,
                            ( #input_idents ): ( #input_types ),
                        ) -> Self::Output {
                            #fn_ident( #input_idents ).await
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
                            _context: &__Context__,
                            _code: ::core::marker::PhantomData<__Code__>,
                            ( #input_idents ): ( #input_types ),
                        ) -> Self::Output {
                            #fn_ident( #input_idents )
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
