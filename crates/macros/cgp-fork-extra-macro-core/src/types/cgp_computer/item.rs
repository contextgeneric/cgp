use cgp_fork_macro_core::functions::{is_implicit_arg, parse_context_arg};
use cgp_fork_macro_core::parse_internal;
use cgp_fork_macro_core::types::implicits::{ImplicitArgField, ImplicitArgFields};
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Error, Expr, FnArg, Ident, ItemFn, Type, parse2};

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
    ///
    /// A `#[implicit]` or `#[field]` parameter is read from a same-named context
    /// field and is not part of the provider's input tuple. Those attributes are
    /// stripped from the function that is emitted.
    pub fn preprocess(&self) -> syn::Result<PreprocessedCgpComputer> {
        let mut item_fn = self.item_fn.clone();
        let sig = &item_fn.sig;

        let provider_ident = match &self.ident {
            Some(ident) => ident.clone(),
            None => derive_provider_ident(&sig.ident),
        };

        let mut input_idents = Punctuated::new();
        let mut input_types = Punctuated::<Type, Comma>::new();
        let mut call_args = Punctuated::<Expr, Comma>::new();
        let mut context_fields = Vec::new();
        let mut explicit_index = 0usize;

        for input in item_fn.sig.inputs.iter_mut() {
            match input {
                FnArg::Receiver(receiver) => {
                    return Err(Error::new_spanned(
                        receiver,
                        "Computer functions cannot have a receiver",
                    ));
                }
                FnArg::Typed(pat_type) => {
                    if is_implicit_arg(pat_type)? {
                        let field: ImplicitArgField = parse_context_arg(pat_type)?;
                        let context: Expr = parse_internal!(context);
                        call_args.push(field.to_expr(&context)?);
                        context_fields.push(field);
                        continue;
                    }

                    if let Some(impl_trait) = find_impl_trait(&pat_type.ty) {
                        return Err(Error::new_spanned(
                            impl_trait,
                            "Computer function parameters cannot use `impl Trait`; declare a generic parameter instead",
                        ));
                    }

                    // Each explicit input is rebound positionally, so the function's
                    // own pattern (a `mut` binding, a destructuring) is not repeated.
                    let ident = Ident::new(&format!("arg_{explicit_index}"), pat_type.span());
                    explicit_index += 1;
                    input_idents.push(ident.clone());
                    input_types.push(pat_type.ty.as_ref().clone());
                    call_args.push(parse_internal!(#ident));
                }
            }
        }

        let output = return_type(&item_fn.sig.output)?;

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
            item_fn,
            input_idents,
            input_types,
            call_args,
            context_fields: ImplicitArgFields::new(context_fields),
            output,
            is_async: self.item_fn.sig.asyncness.is_some(),
            is_fallible: maybe_result.error_type.is_some(),
        })
    }
}
