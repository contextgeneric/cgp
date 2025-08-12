use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse2, Ident, ItemFn, ItemImpl, ReturnType};

use crate::utils::to_camel_case_str;

pub fn cgp_producer(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let item_fn: ItemFn = parse2(body)?;

    let fn_sig = &item_fn.sig;
    let fn_ident = &fn_sig.ident;

    let producer_ident = if attr.is_empty() {
        Ident::new(&to_camel_case_str(&fn_ident.to_string()), fn_ident.span())
    } else {
        parse2(attr)?
    };

    if !fn_sig.inputs.is_empty() {
        return Err(syn::Error::new(
            fn_sig.inputs.span(),
            "Producer functions cannot have parameters",
        ));
    }

    if fn_sig.asyncness.is_some() {
        return Err(syn::Error::new(
            fn_sig.asyncness.span(),
            "Producer functions cannot be async",
        ));
    }

    if !fn_sig.generics.params.is_empty() {
        return Err(syn::Error::new(
            fn_sig.generics.params.span(),
            "Producer functions must have empty generic parameters",
        ));
    }

    let fn_output = match &fn_sig.output {
        ReturnType::Type(_, ty) => ty.as_ref().clone(),
        ReturnType::Default => syn::parse_quote!(()),
    };

    let producer: ItemImpl = parse2(quote! {
        #[cgp_new_provider]
        impl<__Context__, __Code__>
            Producer<__Context__, __Code__>
            for #producer_ident
        {
            type Output = #fn_output;

            fn produce(_context: &__Context__, _code: PhantomData<__Code__>) -> Self::Output {
                #fn_ident()
            }
        }
    })?;

    let delegate = quote! {
        delegate_components! {
            #producer_ident {
                ComputerComponent: Promote<#producer_ident>,
                ComputerRefComponent: PromoteRef<Self>,
                TryComputerComponent: Promote<Self>,
                TryComputerRefComponent: PromoteRef<Self>,
                AsyncComputerComponent: PromoteAsync<Self>,
                AsyncComputerRefComponent: PromoteRef<Self>,
                HandlerComponent: PromoteAsync<Self>,
                HandlerRefComponent: PromoteRef<Self>,
            }
        }
    };

    Ok(quote! {
        #item_fn

        #producer

        #delegate
    })
}
