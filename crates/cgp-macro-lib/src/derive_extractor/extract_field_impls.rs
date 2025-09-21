use quote::quote;
use syn::{Arm, GenericArgument, Ident, ItemEnum, ItemImpl, Type, parse2};

use crate::derive_builder::{index_to_generic_ident, to_generic_args};
use crate::derive_extractor::get_variant_type;
use crate::symbol::symbol_from_string;

pub fn derive_extract_field_impls(
    context_enum: &ItemEnum,
    extractor_ident: &Ident,
    is_ref: bool,
) -> syn::Result<Vec<ItemImpl>> {
    let mut item_impls = Vec::new();

    let generics = {
        let mut generics = context_enum.generics.clone();

        if is_ref {
            generics.params.insert(
                0,
                parse2(quote! {
                    '__a__
                })?,
            );

            generics.params.insert(
                1,
                parse2(quote! {
                    __R__: MapTypeRef
                })?,
            );
        }

        generics
    };

    let base_generic_args = to_generic_args(&generics)?;

    for (current_index, current_variant) in context_enum.variants.iter().enumerate() {
        let mut generics = generics.clone();
        let mut source_generic_args = base_generic_args.args.clone();
        let mut output_generic_args = base_generic_args.args.clone();
        let mut match_arms = Vec::<Arm>::new();

        for (other_index, other_variant) in context_enum.variants.iter().enumerate() {
            let variant_ident = &other_variant.ident;

            if other_index != current_index {
                let generic_param_name = index_to_generic_ident(other_index);

                generics.params.push(parse2(quote! {
                    #generic_param_name: MapType
                })?);

                let generic_arg: GenericArgument = parse2(quote! { #generic_param_name })?;
                source_generic_args.push(generic_arg.clone());
                output_generic_args.push(generic_arg);

                match_arms.push(parse2(quote! {
                    #extractor_ident :: #variant_ident ( value ) => {
                        Err(#extractor_ident :: #variant_ident ( value ))
                    }
                })?);
            } else {
                source_generic_args.push(parse2(quote! { IsPresent })?);
                output_generic_args.push(parse2(quote! { IsVoid })?);

                match_arms.push(parse2(quote! {
                    #extractor_ident :: #variant_ident ( value ) => {
                        Ok( value )
                    }
                })?);
            }
        }

        let value_type = {
            let value_type = get_variant_type(current_variant)?;

            if is_ref {
                parse2(quote! { <__R__ as MapTypeRef>::Map<'__a__, #value_type> })?
            } else {
                value_type.clone()
            }
        };

        let tag_type = symbol_from_string(&current_variant.ident.to_string());

        let source_type: Type = parse2(quote! {
            #extractor_ident < #source_generic_args >
        })?;

        let output_type: Type = parse2(quote! {
            #extractor_ident < #output_generic_args >
        })?;

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let item_impl = parse2(quote! {
            impl #impl_generics ExtractField< #tag_type >
                for #source_type
            #where_clause
            {
                type Value = #value_type;

                type Remainder = #output_type;

                fn extract_field(self, _tag: ::core::marker::PhantomData< #tag_type >) -> Result<Self::Value, Self::Remainder> {
                    match self {
                        #(#match_arms)*
                    }
                }
            }
        })?;

        item_impls.push(item_impl);
    }

    Ok(item_impls)
}
