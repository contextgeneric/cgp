use quote::quote;
use syn::{Generics, Ident, ItemFn, ItemTrait, TraitItemFn, parse2};

pub fn derive_item_trait(
    trait_ident: &Ident,
    item_fn: &ItemFn,
    generics: &Generics,
) -> syn::Result<ItemTrait> {
    let trait_item_fn = TraitItemFn {
        attrs: item_fn.attrs.clone(),
        sig: item_fn.sig.clone(),
        default: None,
        semi_token: None,
    };

    let (_, type_generics, _) = generics.split_for_impl();

    let item_trait: ItemTrait = parse2(quote! {
        pub trait #trait_ident #type_generics {
            #trait_item_fn
        }
    })?;

    Ok(item_trait)
}
