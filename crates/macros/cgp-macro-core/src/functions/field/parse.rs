use quote::ToTokens;
use syn::spanned::Spanned;
use syn::token::Mut;
use syn::{Error, GenericArgument, PathArguments, PathSegment, Type, TypePath};

use crate::functions::parse_internal;
use crate::types::getter::FieldMode;

pub fn parse_field_type(
    return_type: &Type,
    receiver_mut: &Option<Mut>,
) -> syn::Result<(Type, FieldMode)> {
    match &return_type {
        Type::Reference(type_ref) => {
            if type_ref.mutability.is_some() && receiver_mut.is_none() {
                return Err(Error::new(
                    type_ref.span(),
                    format!(
                        "&mut self is required for mutable field reference `{}`",
                        type_ref.to_token_stream()
                    ),
                ));
            }

            if type_ref.elem.as_ref() == &parse_internal! { str } {
                // Special case to handle &str as String field

                let field_type: Type = parse_internal! { String };

                Ok((field_type, FieldMode::Str))
            } else if let (Type::Slice(slice), None) = (type_ref.elem.as_ref(), receiver_mut) {
                let field_type = slice.elem.as_ref().clone();

                Ok((field_type, FieldMode::Slice))
            } else {
                let field_type = type_ref.elem.as_ref().clone();

                Ok((field_type, FieldMode::Reference))
            }
        }
        Type::Path(type_path) => {
            if let Some(field_type) = try_parse_option_ref(type_path) {
                Ok((
                    parse_internal! { Option< #field_type > },
                    FieldMode::OptionRef,
                ))
            } else if let (Some(field_type), None) = (try_parse_mref(type_path), receiver_mut) {
                Ok((field_type.clone(), FieldMode::MRef))
            } else {
                Ok((return_type.clone(), FieldMode::Copy))
            }
        }
        _ => Err(Error::new(
            return_type.span(),
            "return type must be a reference",
        )),
    }
}

fn try_parse_option_ref(type_path: &TypePath) -> Option<&Type> {
    let segment = parse_single_segment_type_path(type_path).ok()?;

    if segment.ident == "Option"
        && let PathArguments::AngleBracketed(args) = &segment.arguments
    {
        let [arg] = Vec::from_iter(args.args.iter()).try_into().ok()?;

        if let GenericArgument::Type(Type::Reference(type_ref)) = arg {
            return Some(type_ref.elem.as_ref());
        }
    }

    None
}

pub fn parse_single_segment_type_path(type_path: &TypePath) -> syn::Result<&PathSegment> {
    let [segment]: [&PathSegment; 1] = type_path
        .path
        .segments
        .iter()
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| {
            Error::new(
                type_path.span(),
                "type path must contain exactly one path segment",
            )
        })?;

    Ok(segment)
}

fn try_parse_mref(type_path: &TypePath) -> Option<&Type> {
    let segment = parse_single_segment_type_path(type_path).ok()?;

    if segment.ident == "MRef"
        && let PathArguments::AngleBracketed(args) = &segment.arguments
    {
        let [arg1, arg2] = Vec::from_iter(args.args.iter()).try_into().ok()?;

        if let (GenericArgument::Lifetime(_), GenericArgument::Type(ty)) = (arg1, arg2) {
            return Some(ty);
        }
    }

    None
}
