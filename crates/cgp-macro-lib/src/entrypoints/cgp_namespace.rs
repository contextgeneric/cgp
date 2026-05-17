use proc_macro2::TokenStream;
use syn::parse2;

use crate::cgp_namespace::{NamespaceSpec, derive_namespace};

pub fn cgp_namespace(body: TokenStream) -> syn::Result<TokenStream> {
    let spec: NamespaceSpec = parse2(body)?;

    derive_namespace(spec)
}
