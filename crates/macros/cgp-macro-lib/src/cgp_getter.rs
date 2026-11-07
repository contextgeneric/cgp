use cgp_macro_core::types::cgp_component::{CgpComponentRawArgs, ItemCgpComponent};
use cgp_macro_core::types::cgp_getter::ItemCgpGetter;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemTrait, parse2};

/// Define a getter as a full CGP component, so the field it reads is chosen when a context is wired rather than fixed to the method name.
///
/// See [`cgp_getter`](https://contextgeneric.dev/docs/reference/macros/cgp_getter).
pub fn cgp_getter(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let mut raw_args: CgpComponentRawArgs = parse2(attr.clone())?;

    let item_trait: ItemTrait = syn::parse2(body)?;

    if raw_args.provider_ident.is_none()
        && let Some(field_name) = item_trait.ident.to_string().strip_prefix("Has")
        && !field_name.is_empty()
    {
        raw_args.provider_ident = Some(Ident::new(
            &format!("{field_name}Getter"),
            item_trait.ident.span(),
        ));
    }

    let item_cgp_component = ItemCgpComponent {
        args: raw_args.try_into()?,
        item_trait,
    };

    let evaluated = item_cgp_component.preprocess()?.eval()?;

    let item_getter = ItemCgpGetter::try_from(evaluated)?;

    let items = item_getter.to_items()?;

    let derived = quote! {
        #( #items )*
    };

    Ok(derived)
}
