use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{parse2, ItemStruct};

use crate::derive_has_fields::derive_has_fields_impls_from_struct;

pub fn derive_has_fields(body: TokenStream) -> syn::Result<TokenStream> {
    let item_struct: ItemStruct = parse2(body)?;
    let impls = derive_has_fields_impls_from_struct(&item_struct)?;

    let mut out = TokenStream::new();

    out.append_all(impls);

    Ok(out)
}
