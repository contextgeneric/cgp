use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{parse2, FnArg, Ident, ItemFn, ItemImpl, ReturnType, Type};

use crate::parse::MaybeResultType;
use crate::utils::to_camel_case_str;

pub fn cgp_computer(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let item_fn: ItemFn = parse2(body)?;

    let fn_sig = &item_fn.sig;
    let fn_ident = &fn_sig.ident;
    let fn_inputs = &fn_sig.inputs;

    let computer_ident = if attr.is_empty() {
        Ident::new(&to_camel_case_str(&fn_ident.to_string()), fn_ident.span())
    } else {
        parse2(attr)?
    };

    if fn_sig.asyncness.is_some() {
        return Err(syn::Error::new(
            fn_sig.asyncness.span(),
            "Computer functions cannot be async",
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
                input_idents.push(Ident::new(&format!("arg_{i}"), pat.span()));
            }
        }
    }

    let mut generics = fn_sig.generics.clone();
    generics.params.push(parse2(quote! { __Context__ })?);
    generics.params.push(parse2(quote! { __Code__ })?);

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let fn_output = match &fn_sig.output {
        ReturnType::Type(_, ty) => ty.as_ref().clone(),
        ReturnType::Default => syn::parse_quote!(()),
    };

    let computer: ItemImpl = parse2(quote! {
        #[cgp_new_provider]
        impl #impl_generics
            Computer<__Context__, __Code__, ( #input_types )>
            for #computer_ident
        #where_clause
        {
            type Output = #fn_output;

            fn compute(_context: &__Context__, _code: PhantomData<__Code__>, ( #input_idents ): ( #input_types )) -> Self::Output {
                #fn_ident( #input_idents )
            }
        }
    })?;

    let maybe_result_type = parse2::<MaybeResultType>(fn_output.to_token_stream())?;

    let try_computer = if maybe_result_type.error_type.is_some() {
        quote!(TryPromote< #computer_ident >)
    } else {
        quote!(Promote< #computer_ident >)
    };

    let delegate = quote! {
        delegate_components! {
            #computer_ident {
                TryComputerComponent: #try_computer,
                HandlerComponent: Promote<#try_computer>,
            }
        }
    };

    Ok(quote! {
        #item_fn

        #computer

        #delegate
    })
}
