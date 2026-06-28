use cgp_macro_core::types::cgp_data::get_variant_type;
use cgp_macro_core::types::field::Symbol;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, ItemImpl, parse2};

pub fn derive_from_variant(body: TokenStream) -> syn::Result<TokenStream> {
    let item_enum: ItemEnum = parse2(body)?;

    derive_from_variant_from_enum(&item_enum)
}

pub fn derive_from_variant_from_enum(item_enum: &ItemEnum) -> syn::Result<TokenStream> {
    let enum_ident = &item_enum.ident;

    let (impl_generics, ty_generics, where_clause) = item_enum.generics.split_for_impl();

    let mut item_impls: Vec<ItemImpl> = Vec::new();

    for variant in item_enum.variants.iter() {
        let variant_ident = &variant.ident;
        let variant_tag = Symbol::from_ident(variant_ident.clone());
        let variant_type = get_variant_type(variant)?;

        let item_impl: ItemImpl = parse2(quote! {
            impl #impl_generics FromVariant<#variant_tag> for #enum_ident #ty_generics
            #where_clause
            {
                type Value = #variant_type;

                fn from_variant(_tag: ::core::marker::PhantomData<#variant_tag>, value: Self::Value) -> Self {
                    Self::#variant_ident(value)
                }
            }
        })?;

        item_impls.push(item_impl);
    }

    let out = quote! {
        #(#item_impls)*
    };

    Ok(out)
}
