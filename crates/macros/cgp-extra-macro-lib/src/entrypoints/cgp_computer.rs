use cgp_macro_core::functions::{is_implicit_arg, parse_context_arg, to_camel_case_str};
use cgp_macro_core::traits::ToTypeParamBounds;
use cgp_macro_core::types::implicits::{ImplicitArgField, ImplicitArgFields};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Expr, FnArg, Ident, ItemFn, ItemImpl, ReturnType, Type, parse2};

use crate::parse::MaybeResultType;

/// A `#[cgp_computer]` parameter: either an explicit input or a value read from
/// a same-named context field (`#[implicit]` or `#[field]`).
enum ComputerArg {
    Context(Box<ImplicitArgField>),
    Value { ty: Box<Type>, ident: Ident },
}

pub fn cgp_computer(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let mut item_fn: ItemFn = parse2(body)?;

    let fn_sig = &item_fn.sig;
    let fn_ident = &fn_sig.ident;

    let computer_ident = if attr.is_empty() {
        Ident::new(&to_camel_case_str(&fn_ident.to_string()), fn_ident.span())
    } else {
        parse2(attr)?
    };

    let args = classify_args(&mut item_fn.sig.inputs)?;
    let context_fields = context_fields(&args);

    let mut input_types = Punctuated::<Type, Comma>::new();
    let mut input_idents = Punctuated::<Ident, Comma>::new();

    for arg in &args {
        if let ComputerArg::Value { ty, ident } = arg {
            input_types.push(ty.as_ref().clone());
            input_idents.push(ident.clone());
        }
    }

    let fn_output = match &item_fn.sig.output {
        ReturnType::Type(_, ty) => ty.as_ref().clone(),
        ReturnType::Default => syn::parse_quote!(()),
    };

    let maybe_result_type = parse2::<MaybeResultType>(fn_output.to_token_stream())?;

    let context_ident = if context_fields.fields.is_empty() {
        Ident::new("_context", proc_macro2::Span::call_site())
    } else {
        Ident::new("context", proc_macro2::Span::call_site())
    };

    let call_args = call_arguments(&args)?;
    let fn_ident = &item_fn.sig.ident;

    let mut generics = item_fn.sig.generics.clone();
    generics.params.push(parse2(quote! { __Context__ })?);
    generics.params.push(parse2(quote! { __Code__ })?);

    if !context_fields.fields.is_empty() {
        let bounds = context_fields.to_type_param_bounds()?;
        generics
            .make_where_clause()
            .predicates
            .push(parse2(quote! { __Context__: #bounds })?);
    }

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    if item_fn.sig.asyncness.is_none() {
        let computer: ItemImpl = parse2(quote! {
            #[cgp_new_provider]
            impl #impl_generics
                Computer<__Context__, __Code__, ( #input_types )>
                for #computer_ident
            #where_clause
            {
                type Output = #fn_output;

                fn compute(#context_ident: &__Context__, _code: PhantomData<__Code__>, ( #input_idents ): ( #input_types )) -> Self::Output {
                    #fn_ident( #call_args )
                }
            }
        })?;

        let delegate = if maybe_result_type.error_type.is_some() {
            quote! {
                delegate_components! {
                    #computer_ident {
                        [
                            ComputerRefComponent,
                            TryComputerComponent,
                            TryComputerRefComponent,
                            AsyncComputerComponent,
                            AsyncComputerRefComponent,
                            HandlerComponent,
                            HandlerRefComponent,
                        ] ->
                            PromoteTryComputer<Self>,
                    }
                }
            }
        } else {
            quote! {
                delegate_components! {
                    #computer_ident {
                        [
                            ComputerRefComponent,
                            TryComputerComponent,
                            TryComputerRefComponent,
                            AsyncComputerComponent,
                            AsyncComputerRefComponent,
                            HandlerComponent,
                            HandlerRefComponent,
                        ] ->
                            PromoteComputer<Self>,
                    }
                }
            }
        };

        Ok(quote! {
            #item_fn

            #computer

            #delegate
        })
    } else {
        let computer: ItemImpl = parse2(quote! {
            #[cgp_new_provider]
            impl #impl_generics
                AsyncComputer<__Context__, __Code__, ( #input_types )>
                for #computer_ident
            #where_clause
            {
                type Output = #fn_output;

                async fn compute_async(
                    #context_ident: &__Context__,
                    _code: PhantomData<__Code__>,
                    ( #input_idents ): ( #input_types )
                ) -> Self::Output {
                    #fn_ident( #call_args ).await
                }
            }
        })?;

        let delegate_ref = if maybe_result_type.error_type.is_some() {
            quote! {
                delegate_components! {
                    #computer_ident {
                        [
                            AsyncComputerRefComponent,
                            HandlerComponent,
                            HandlerRefComponent,
                        ] ->
                            PromoteHandler<Self>,
                    }
                }
            }
        } else {
            quote! {
                delegate_components! {
                    #computer_ident {
                        [
                            AsyncComputerRefComponent,
                            HandlerComponent,
                            HandlerRefComponent,
                        ] ->
                            PromoteAsyncComputer<Self>,
                    }
                }
            }
        };

        Ok(quote! {
            #item_fn

            #computer

            #delegate_ref
        })
    }
}

fn classify_args(inputs: &mut Punctuated<FnArg, Comma>) -> syn::Result<Vec<ComputerArg>> {
    let mut args = Vec::new();
    let mut explicit_index = 0usize;

    for input in inputs.iter_mut() {
        match input {
            FnArg::Receiver(_) => {
                return Err(syn::Error::new(
                    input.span(),
                    "Computer functions cannot have a receiver",
                ));
            }
            FnArg::Typed(pat) => {
                if is_implicit_arg(pat)? {
                    args.push(ComputerArg::Context(Box::new(parse_context_arg(pat)?)));
                } else {
                    let ident = Ident::new(&format!("arg_{explicit_index}"), pat.span());
                    explicit_index += 1;
                    args.push(ComputerArg::Value {
                        ty: Box::new(pat.ty.as_ref().clone()),
                        ident,
                    });
                }
            }
        }
    }

    Ok(args)
}

fn context_fields(args: &[ComputerArg]) -> ImplicitArgFields {
    let fields = args
        .iter()
        .filter_map(|arg| match arg {
            ComputerArg::Context(field) => Some(field.as_ref().clone()),
            ComputerArg::Value { .. } => None,
        })
        .collect();

    ImplicitArgFields::new(fields)
}

fn call_arguments(args: &[ComputerArg]) -> syn::Result<Punctuated<Expr, Comma>> {
    let context: Expr = parse2(quote!(context))?;
    let mut call_args = Punctuated::new();

    for arg in args {
        match arg {
            ComputerArg::Context(field) => call_args.push(field.to_expr(&context)?),
            ComputerArg::Value { ident, .. } => call_args.push(parse2(quote!(#ident))?),
        }
    }

    Ok(call_args)
}
