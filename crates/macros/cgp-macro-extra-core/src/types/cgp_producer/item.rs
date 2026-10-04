use syn::{Error, Ident, ItemFn};

use crate::functions::{derive_provider_ident, return_type};
use crate::types::cgp_producer::PreprocessedCgpProducer;
use crate::visitors::find_impl_trait;

/// Raw input stage: the optional provider name from the attribute and the
/// annotated function. First stage of the `#[cgp_producer]` pipeline.
pub struct ItemCgpProducer {
    pub ident: Option<Ident>,
    pub item_fn: ItemFn,
}

impl ItemCgpProducer {
    /// Check the function has a producer's shape and resolve the provider name
    /// and output type, yielding the next stage.
    pub fn preprocess(&self) -> syn::Result<PreprocessedCgpProducer> {
        self.check_no_inputs()?;
        self.check_not_async()?;
        self.check_no_generics()?;

        let sig = &self.item_fn.sig;

        let provider_ident = match &self.ident {
            Some(ident) => ident.clone(),
            None => derive_provider_ident(&sig.ident),
        };

        let output = return_type(&sig.output)?;

        if let Some(impl_trait) = find_impl_trait(&output) {
            return Err(Error::new_spanned(
                impl_trait,
                "Producer functions cannot return `impl Trait`",
            ));
        }

        Ok(PreprocessedCgpProducer {
            provider_ident,
            item_fn: self.item_fn.clone(),
            output,
        })
    }

    /// Reject any parameter, a receiver included: `Producer::produce` receives
    /// only the context and the code tag, so there is no input to pass on.
    fn check_no_inputs(&self) -> syn::Result<()> {
        let inputs = &self.item_fn.sig.inputs;

        if inputs.is_empty() {
            Ok(())
        } else {
            Err(Error::new_spanned(
                inputs,
                "Producer functions cannot have parameters",
            ))
        }
    }

    /// Reject `async`: the `Producer` trait is synchronous.
    fn check_not_async(&self) -> syn::Result<()> {
        match &self.item_fn.sig.asyncness {
            None => Ok(()),
            Some(asyncness) => Err(Error::new_spanned(
                asyncness,
                "Producer functions cannot be async",
            )),
        }
    }

    /// Reject generic parameters, lifetimes included. The generated impl's only
    /// parameters are the reserved context and code, and the function's own
    /// parameters would appear in neither its trait arguments nor its self type,
    /// so the compiler would reject them as unconstrained (`E0207`).
    fn check_no_generics(&self) -> syn::Result<()> {
        let params = &self.item_fn.sig.generics.params;

        if params.is_empty() {
            Ok(())
        } else {
            Err(Error::new_spanned(
                params,
                "Producer functions must have empty generic parameters",
            ))
        }
    }
}
