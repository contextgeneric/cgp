use cgp_macro_core::parse_internal;
use cgp_macro_core::types::cgp_provider::{ItemCgpProvider, ProviderArgs};
use cgp_macro_core::types::delegate_component::DelegateTable;
use cgp_macro_core::types::keyword::Keyword;
use syn::{Ident, ItemFn, ItemImpl, Type};

use crate::exports::{
    AsyncComputerComponent, AsyncComputerRefComponent, ComputerComponent, ComputerRefComponent,
    HandlerComponent, HandlerRefComponent, Producer, ProducerComponent, PromoteProducer,
    TryComputerComponent, TryComputerRefComponent,
};
use crate::types::handler_fn::EvaluatedHandlerFn;

/// The validated `#[cgp_producer]` input: the resolved provider name, the
/// function, and its output type.
pub struct PreprocessedCgpProducer {
    pub provider_ident: Ident,
    pub item_fn: ItemFn,
    pub output: Type,
}

impl PreprocessedCgpProducer {
    /// Evaluate into the shared handler-function IR: a `Producer` impl that calls
    /// the function, and a table routing every handler component to
    /// `PromoteProducer<Self>`, which lets each handler shape yield the produced
    /// value whatever input it is given.
    pub fn eval(&self) -> syn::Result<EvaluatedHandlerFn> {
        let provider_ident = &self.provider_ident;
        let fn_ident = &self.item_fn.sig.ident;
        let output = &self.output;

        let item_impl: ItemImpl = parse_internal! {
            impl<__Context__, __Code__> #Producer<__Context__, __Code__> for #provider_ident {
                type Output = #output;

                fn produce(
                    _context: &__Context__,
                    _code: ::core::marker::PhantomData<__Code__>,
                ) -> Self::Output {
                    #fn_ident()
                }
            }
        };

        let provider = ItemCgpProvider {
            args: ProviderArgs {
                new: Some(Keyword::default()),
                component_type: Some(parse_internal!(#ProducerComponent)),
            },
            item_impl,
        };

        let delegate_table: DelegateTable = parse_internal! {
            #provider_ident {
                [
                    #ComputerComponent,
                    #ComputerRefComponent,
                    #TryComputerComponent,
                    #TryComputerRefComponent,
                    #AsyncComputerComponent,
                    #AsyncComputerRefComponent,
                    #HandlerComponent,
                    #HandlerRefComponent,
                ]:
                    #PromoteProducer<Self>,
            }
        };

        Ok(EvaluatedHandlerFn {
            item_fn: self.item_fn.clone(),
            provider,
            delegate_table,
        })
    }
}
