use proc_macro2::{Span, TokenStream};
use quote::{TokenStreamExt, quote};
use syn::token::Pub;
use syn::{Attribute, Ident, Item, ItemMod, ItemUse, Visibility, parse_quote, parse2};

pub fn re_export_imports(attrs: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let export_mod_name: Ident = if !attrs.is_empty() {
        parse2(attrs)?
    } else {
        Ident::new("re_exports", Span::call_site())
    };

    let mut re_exports: Vec<ItemUse> = Vec::new();

    let item_mod: ItemMod = parse2(body)?;

    let mod_name = &item_mod.ident;

    let doc_hidden: Attribute = parse_quote! { #[doc(hidden)] };
    let doc_no_inline: Attribute = parse_quote! { #[doc(no_inline)] };

    if let Some(content) = &item_mod.content {
        for item in content.1.iter() {
            if let Item::Use(use_item) = item {
                let mut re_export = use_item.clone();

                re_export.vis = Visibility::Public(Pub(Span::call_site()));
                re_export.attrs.push(doc_hidden.clone());
                re_export.attrs.push(doc_no_inline.clone());

                re_exports.push(re_export);
            }
        }
    }

    let mut mod_body = TokenStream::new();
    mod_body.append_all(re_exports);

    let export_mod: ItemMod = parse2(quote! {
        #[doc(hidden)]
        #[allow(unused_imports)]
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
