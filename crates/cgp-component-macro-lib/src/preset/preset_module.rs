use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::token::Pub;
use syn::{parse2, parse_quote, Attribute, Ident, Item, ItemMod, ItemUse, Visibility};

use crate::define_preset;

pub fn derive_preset_module(attrs: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let export_mod_name: Ident = if !attrs.is_empty() {
        parse2(attrs)?
    } else {
        Ident::new("re_exports", Span::call_site())
    };

    let mut re_exports: Vec<ItemUse> = Vec::new();

    let mut item_mod: ItemMod = parse2(body)?;

    let mod_name = &item_mod.ident;

    let doc_hidden: Attribute = parse_quote! { #[doc(hidden)] };
    let doc_no_inline: Attribute = parse_quote! { #[doc(no_inline)] };

    if let Some(content) = &mut item_mod.content {
        for item in content.1.iter_mut() {
            match item {
                Item::Use(use_item) => {
                    let mut re_export = use_item.clone();

                    re_export.vis = Visibility::Public(Pub(Span::call_site()));
                    re_export.attrs.push(doc_hidden.clone());
                    re_export.attrs.push(doc_no_inline.clone());

                    re_exports.push(re_export);
                }
                Item::Macro(macro_item) => {
                    if macro_item.mac.path == parse_quote!(cgp_preset) {
                        let macro_body = macro_item.mac.tokens.clone();
                        let new_body = define_preset(macro_body)?;
                    }
                }
                _ => {}
            }
        }
    }

    let mut mod_body = TokenStream::new();
    mod_body.append_all(re_exports);

    let export_mod: ItemMod = parse2(quote! {
        #[doc(hidden)]
        mod #export_mod_name {
            #mod_body
        }
    })?;

    let out = quote! {
        #item_mod

        #export_mod

        pub use #mod_name ::*;
    };

    Ok(out)
}
