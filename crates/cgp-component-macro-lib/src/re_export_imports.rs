use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::token::Pub;
use syn::{parse2, parse_quote, Error, Item, ItemMod, Visibility};

pub fn derive_re_export_imports(attrs: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    if !attrs.is_empty() {
        return Err(Error::new_spanned(
            attrs,
            "#[re_export_imports] do not accept any attribute argument",
        ));
    }

    let mut item_mod: ItemMod = parse2(body)?;

    let mod_name = &item_mod.ident;

    if let Some(content) = &mut item_mod.content {
        for item in content.1.iter_mut() {
            if let Item::Use(use_item) = item {
                match &use_item.vis {
                    Visibility::Public(_) => {}
                    _ => {
                        use_item.vis = Visibility::Public(Pub(Span::call_site()));
                        use_item.attrs.push(parse_quote!( #[doc(hidden)] ));
                    }
                }
            }
        }
    }

    let out = quote! {
        #item_mod

        pub use #mod_name ::*;
    };

    Ok(out)
}
