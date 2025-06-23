use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{parse2, parse_quote, FnArg, Ident, ItemFn, ItemImpl, ReturnType, Type};

use crate::parse::MaybeResultType;
use crate::utils::to_camel_case_str;

pub fn cgp_handler(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let item_fn: ItemFn = parse2(body)?;

    let fn_sig = &item_fn.sig;
    let fn_ident = &fn_sig.ident;
    let fn_inputs = &fn_sig.inputs;

    let handler_ident = if attr.is_empty() {
        Ident::new(&to_camel_case_str(&fn_ident.to_string()), fn_ident.span())
    } else {
        parse2(attr)?
    };

    if fn_sig.asyncness.is_none() {
        return Err(syn::Error::new(
            fn_sig.asyncness.span(),
            "Handler functions must be async",
        ));
    }

    let mut input_types = Punctuated::<Type, Comma>::new();
    let mut input_idents = Punctuated::<Ident, Comma>::new();

    for (i, input) in fn_inputs.iter().enumerate() {
        match input {
            FnArg::Receiver(_) => {
                return Err(syn::Error::new(
                    input.span(),
                    "Computer functions cannot have a receiver",
                ));
            }
            FnArg::Typed(pat) => {
                input_types.push(pat.ty.as_ref().clone());
                input_idents.push(Ident::new(&format!("arg_{}", i), pat.span()));
            }
        }
    }

    let mut generics = fn_sig.generics.clone();
    generics.params.push(parse2(quote! { __Context__ })?);
    generics.params.push(parse2(quote! { __Code__: Send })?);

    let fn_output = match &fn_sig.output {
        ReturnType::Type(_, ty) => ty.as_ref().clone(),
        ReturnType::Default => syn::parse_quote!(()),
    };

    let maybe_result_type = parse2::<MaybeResultType>(fn_output.to_token_stream())?;

    let where_clause = generics.make_where_clause();

    if let Some(error_type) = &maybe_result_type.error_type {
        where_clause.predicates.push(parse_quote! {
            __Context__: CanRaiseAsyncError< #error_type >
        });
    } else {
        where_clause.predicates.push(parse_quote! {
            __Context__: HasAsyncErrorType
        });
    }

    let body = quote! {
        #fn_ident( #input_idents ).await
    };

    let body = if maybe_result_type.error_type.is_some() {
        quote! {
            #body .map_err(__Context__::raise_error)
        }
    } else {
        quote! {
            Ok( #body )
        }
    };

    let output_type = maybe_result_type.success_type;

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let computer: ItemImpl = parse2(quote! {
        #[cgp_new_provider]
        impl #impl_generics
            Handler<__Context__, __Code__, ( #input_types )>
            for #handler_ident
        #where_clause
        {
            type Output = #output_type;

            async fn handle(
                _context: &__Context__,
                _code: PhantomData<__Code__>,
                ( #input_idents ): ( #input_types )
            ) -> Result<Self::Output, __Context__::Error> {
                #body
            }
        }
    })?;

    Ok(quote! {
        #item_fn

        #computer
    })
}
