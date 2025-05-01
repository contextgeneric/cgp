use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{parse2, Error, Item};

use crate::derive_has_fields::{
    derive_has_fields_impls_from_enum, derive_has_fields_impls_from_struct,
};

pub fn derive_has_fields(body: TokenStream) -> syn::Result<TokenStream> {
    let item: Item = parse2(body)?;

    let impls = match item {
        Item::Struct(item_struct) => derive_has_fields_impls_from_struct(&item_struct)?,
        Item::Enum(item_enum) => derive_has_fields_impls_from_enum(&item_enum)?,
        _ => {
            return Err(Error::new_spanned(
                item,
                "expect body to be either a struct or enum",
            ));
        }
    };

    let mut out = TokenStream::new();

    out.append_all(impls);

    Ok(out)
}
