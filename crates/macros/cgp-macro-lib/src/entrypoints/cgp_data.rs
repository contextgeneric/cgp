use proc_macro2::TokenStream;
use syn::{Error, Item, parse2};

use crate::{derive_cgp_record_from_struct, derive_cgp_variant_from_enum};

pub fn derive_cgp_data(body: TokenStream) -> syn::Result<TokenStream> {
    let item: Item = parse2(body)?;

    match item {
        Item::Struct(item_struct) => derive_cgp_record_from_struct(&item_struct),
        Item::Enum(item_enum) => derive_cgp_variant_from_enum(&item_enum),
        _ => Err(Error::new_spanned(
            item,
            "expect body to be either a struct or enum",
        )),
    }
}
