use proc_macro2::TokenStream;
use quote::quote;

use crate::extra_macro_core::types::handler_fn::EvaluatedHandlerFn;

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
