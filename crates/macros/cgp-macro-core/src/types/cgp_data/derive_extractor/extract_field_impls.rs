use syn::{Arm, GenericArgument, Ident, ItemEnum, ItemImpl, Type};

use crate::exports::{ExtractField, IsPresent, IsVoid, MapType, MapTypeRef};
use crate::functions::override_item_span;
use crate::parse_internal;
use crate::types::cgp_data::{get_variant_type, index_to_generic_ident, to_generic_args};
use crate::types::field::Symbol;

/// Emit one `ExtractField` impl per variant on the partial enum, in scope only
/// when that variant's marker is `IsPresent`: it returns `Ok(value)` on a match
/// and `Err(remainder)` — the same partial enum with that variant flipped to
/// `IsVoid` — on a miss. `is_ref` selects the borrowed partial enum.
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
                parse_internal! {
                    '__a__
                },
            );

            generics.params.insert(
                1,
                parse_internal! {
                    __R__: #MapTypeRef
                },
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

                generics.params.push(parse_internal! {
                    #generic_param_name: #MapType
                });

                let generic_arg: GenericArgument = parse_internal! { #generic_param_name };
                source_generic_args.push(generic_arg.clone());
                output_generic_args.push(generic_arg);

                match_arms.push(parse_internal! {
                    #extractor_ident :: #variant_ident ( value ) => {
                        Err(#extractor_ident :: #variant_ident ( value ))
                    }
                });
            } else {
                source_generic_args.push(parse_internal! { #IsPresent });
                output_generic_args.push(parse_internal! { #IsVoid });

                match_arms.push(parse_internal! {
                    #extractor_ident :: #variant_ident ( value ) => {
                        Ok( value )
                    }
                });
            }
        }

        let value_type = {
            let value_type = get_variant_type(current_variant)?;

            if is_ref {
                parse_internal! { <__R__ as #MapTypeRef>::Map<'__a__, #value_type> }
            } else {
                value_type.clone()
            }
        };

        let tag_type = Symbol::from_ident(current_variant.ident.clone());

        let source_type: Type = parse_internal! {
            #extractor_ident < #source_generic_args >
        };

        let output_type: Type = parse_internal! {
            #extractor_ident < #output_generic_args >
        };

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let item_impl: ItemImpl = parse_internal! {
            impl #impl_generics #ExtractField< #tag_type >
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
        };

        // Point an error on this per-variant impl at the variant the user wrote
        // rather than at the whole derive. See
        // cgp-knowledge-base/cgp/implementation/README.md#spans.
        item_impls.push(override_item_span(
            current_variant.ident.span(),
            &item_impl,
        )?);
    }

    Ok(item_impls)
}
