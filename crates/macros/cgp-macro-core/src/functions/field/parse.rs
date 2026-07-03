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
            } else if let (Type::Slice(slice), None) =
                (type_ref.elem.as_ref(), &type_ref.mutability)
            {
                // A shared `&[T]` reads any `AsRef<[T]>` field. Whether the read is
                // mutable follows the reference's *own* mutability, not the
                // receiver's — a `&self`/`&mut self` method may take an immutable
                // slice either way, and there is no `AsMut<[T]>` counterpart, so a
                // `&mut [T]` falls through to the plain reference case below.
                let field_type = slice.elem.as_ref().clone();

                Ok((field_type, FieldMode::Slice))
            } else {
                let field_type = type_ref.elem.as_ref().clone();

                Ok((field_type, FieldMode::Reference))
            }
        }
        Type::Path(type_path) => {
            if let Some(field_type) = try_parse_option_ref(type_path) {
                if field_type == &parse_internal! { str } {
                    // `Option<&str>` is backed by an `Option<String>` field and read
                    // with `.as_deref()`, mirroring the `&str`/`String` special case
                    // for a plain reference.
                    Ok((parse_internal! { Option< String > }, FieldMode::OptionStr))
                } else {
                    Ok((
                        parse_internal! { Option< #field_type > },
                        FieldMode::OptionRef,
                    ))
                }
            } else if let Some(field_type) = try_parse_mref(type_path) {
                // `MRef` borrows the field as a shared value, so — unlike a `&mut`
                // reference — its access mode never depends on the receiver.
                Ok((field_type.clone(), FieldMode::MRef))
            } else {
                Ok((return_type.clone(), FieldMode::Copy))
            }
        }
        // Any other owned type (a tuple, an array, and so on) is read by value and
        // cloned, exactly like an owned path type.
        _ => Ok((return_type.clone(), FieldMode::Copy)),
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
