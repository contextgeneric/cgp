use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Error, FnArg, Ident, ItemFn, parse2};

use crate::functions::{derive_provider_ident, return_type};
use crate::types::cgp_computer::{MaybeResultType, PreprocessedCgpComputer};
use crate::visitors::find_impl_trait;

/// Raw input stage: the optional provider name from the attribute and the
/// annotated function. First stage of the `#[cgp_computer]` pipeline.
pub struct ItemCgpComputer {
    pub ident: Option<Ident>,
    pub item_fn: ItemFn,
}

impl ItemCgpComputer {
    /// Read the function's inputs, output, and whether it is async or returns
    /// `Result<T, E>`, yielding the next stage.
    pub fn preprocess(&self) -> syn::Result<PreprocessedCgpComputer> {
        let sig = &self.item_fn.sig;

        let provider_ident = match &self.ident {
            Some(ident) => ident.clone(),
            None => derive_provider_ident(&sig.ident),
        };

        let mut input_idents = Punctuated::new();
        let mut input_types = Punctuated::new();

        for (i, input) in sig.inputs.iter().enumerate() {
            match input {
                FnArg::Receiver(receiver) => {
                    return Err(Error::new_spanned(
                        receiver,
                        "Computer functions cannot have a receiver",
                    ));
                }
                FnArg::Typed(pat_type) => {
                    if let Some(impl_trait) = find_impl_trait(&pat_type.ty) {
                        return Err(Error::new_spanned(
                            impl_trait,
                            "Computer function parameters cannot use `impl Trait`; declare a generic parameter instead",
                        ));
                    }

                    // Each input is rebound positionally, so the function's own
                    // pattern (a `mut` binding, a destructuring) is not repeated.
                    input_idents.push(Ident::new(&format!("arg_{i}"), pat_type.span()));
                    input_types.push(pat_type.ty.as_ref().clone());
                }
            }
        }

        let output = return_type(&sig.output)?;

        if let Some(impl_trait) = find_impl_trait(&output) {
            return Err(Error::new_spanned(
                impl_trait,
                "Computer functions cannot return `impl Trait`",
            ));
        }

        // A re-parse of the user's own return type, so its tokens keep their spans
        // and an error lands on the offending token.
        let maybe_result: MaybeResultType = parse2(output.to_token_stream())?;

        Ok(PreprocessedCgpComputer {
            provider_ident,
            item_fn: self.item_fn.clone(),
            input_idents,
            input_types,
            output,
            is_async: sig.asyncness.is_some(),
            is_fallible: maybe_result.error_type.is_some(),
        })
    }
}
