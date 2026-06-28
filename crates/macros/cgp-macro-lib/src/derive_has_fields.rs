use cgp_macro_core::types::cgp_data::{ItemCgpRecord, derive_has_fields_impls_from_enum};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Item, parse2};

pub fn derive_has_fields(body: TokenStream) -> syn::Result<TokenStream> {
    let item: Item = parse2(body)?;

    let impls = match item {
        Item::Struct(item_struct) => {
            let record = ItemCgpRecord { item_struct };

            record.to_has_fields_impls()?
        }
        Item::Enum(item_enum) => derive_has_fields_impls_from_enum(&item_enum)?,
        _ => {
            return Err(Error::new_spanned(
                item,
                "expect body to be either a struct or enum",
            ));
        }
    };

    Ok(quote! {
        #( #impls )*
    })
}
