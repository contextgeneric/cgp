use syn::{ItemEnum, ItemImpl};

use crate::exports::FromVariant;
use crate::functions::override_item_span;
use crate::parse_internal;
use crate::types::cgp_data::get_variant_type;
use crate::types::field::Symbol;

/// Emit one `FromVariant` impl per variant, each keyed by the variant name's
/// `Symbol!` and wrapping a payload into that variant. This is the whole of
/// `#[derive(FromVariant)]` and the constructor slice of the variant derives.
pub fn derive_from_variant_from_enum(item_enum: &ItemEnum) -> syn::Result<Vec<ItemImpl>> {
    let enum_ident = &item_enum.ident;

    let (impl_generics, ty_generics, where_clause) = item_enum.generics.split_for_impl();

    let mut item_impls: Vec<ItemImpl> = Vec::new();

    for variant in item_enum.variants.iter() {
        let variant_ident = &variant.ident;
        let variant_tag = Symbol::from_ident(variant_ident.clone());
        let variant_type = get_variant_type(variant)?;

        let item_impl: ItemImpl = parse_internal! {
            impl #impl_generics #FromVariant<#variant_tag> for #enum_ident #ty_generics
            #where_clause
            {
                type Value = #variant_type;

                fn from_variant(_tag: ::core::marker::PhantomData<#variant_tag>, value: Self::Value) -> Self {
                    Self::#variant_ident(value)
                }
            }
        };

        // Aim a compiler error on this impl at the variant the user wrote rather
        // than at the whole `#[derive(...)]`, which is where the impl's
        // `call_site`-spanned boundary would otherwise put the caret. See
        // docs/implementation/README.md#spans.
        item_impls.push(override_item_span(variant_ident.span(), &item_impl)?);
    }

    Ok(item_impls)
}
