use cgp_macro_extra_core::types::handler_fn::EvaluatedHandlerFn;
use proc_macro2::TokenStream;
use quote::quote;

/// Lower the handler-function IR to its final form: the function unchanged, the
/// provider impl with its `IsProviderFor` impl and struct, and the promotion
/// wiring's `DelegateComponent` impls.
pub fn lower_handler_fn(evaluated: &EvaluatedHandlerFn) -> syn::Result<TokenStream> {
    let item_fn = &evaluated.item_fn;
    let provider = evaluated.provider.lower()?;
    let delegate_table = evaluated.delegate_table.eval()?;

    Ok(quote! {
        #item_fn
        #provider
        #delegate_table
    })
}
