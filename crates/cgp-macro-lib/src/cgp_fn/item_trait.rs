use quote::quote;
use syn::{Ident, ItemFn, ItemTrait, TraitItemFn, parse2};

pub fn derive_item_trait(trait_ident: &Ident, item_fn: &ItemFn) -> syn::Result<ItemTrait> {
    let trait_item_fn = TraitItemFn {
        attrs: item_fn.attrs.clone(),
        sig: item_fn.sig.clone(),
        default: None,
        semi_token: None,
    };

    let item_trait: ItemTrait = parse2(quote! {
        pub trait #trait_ident {
            #trait_item_fn
        }
    })?;

    Ok(item_trait)
}
