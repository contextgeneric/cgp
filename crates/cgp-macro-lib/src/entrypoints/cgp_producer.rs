use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse2, Ident, ItemFn, ItemImpl, ReturnType};

pub fn cgp_producer(body: TokenStream) -> syn::Result<TokenStream> {
    let item_fn: ItemFn = parse2(body)?;

    let fn_sig = &item_fn.sig;
    let fn_ident = &fn_sig.ident;

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

    let producer_ident = Ident::new(&to_camel_case_str(&fn_ident.to_string()), fn_ident.span());

    let producer: ItemImpl = parse2(quote! {
        #[cgp_new_provider]
        impl<__Context__, __Code__, __Input__>
            Producer<__Context__, __Code__, __Input__>
            for #producer_ident
        {
            type Output = #fn_output;

            fn produce(_context: &__Context__, _tag: PhantomData<__Code__>) -> Self::Output {
                #fn_ident()
            }
        }
    })?;

    Ok(quote! {
        #item_fn

        #producer
    })
}

fn to_camel_case_str(val: &str) -> String {
    val.split('_')
        .filter(|word| !word.is_empty())
        .flat_map(|word| {
            word.chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        })
        .collect()
}
