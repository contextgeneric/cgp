use quote::ToTokens;
use syn::spanned::Spanned;
use syn::token::Mut;
use syn::{Error, GenericArgument, PathArguments, PathSegment, Type, TypePath, TypeReference};

use crate::functions::parse_internal;
use crate::types::getter::FieldMode;

/// Determine, from the type a field read must produce, the field type to require,
/// the [`FieldMode`] that shapes the read expression, and whether the read borrows
/// the field mutably. The mutability follows the *reference in the type* — the
/// outer `&mut` of a `&mut T`/`&mut [T]`, or the inner `&mut` of an
/// `Option<&mut T>` — and a mutable read always requires a `&mut self` receiver,
/// which `receiver_mut` reports.
pub fn parse_field_type(
    return_type: &Type,
    receiver_mut: &Option<Mut>,
) -> syn::Result<(Type, FieldMode, Option<Mut>)> {
    match &return_type {
        Type::Reference(type_ref) => {
            require_mut_receiver(type_ref, receiver_mut)?;

            let field_mut = type_ref.mutability;

            if type_ref.elem.as_ref() == &parse_internal! { str } {
                // Special case to handle &str as String field

                let field_type: Type = parse_internal! { String };

                Ok((field_type, FieldMode::Str, field_mut))
            } else if let Type::Slice(slice) = type_ref.elem.as_ref() {
                // A shared `&[T]` reads any `AsRef<[T]>` field with `.as_ref()`; a
                // `&mut [T]` reads any `AsMut<[T]>` field with `.as_mut()`. Both use
                // `FieldMode::Slice`; the mutability carried alongside selects the
                // bound and the read method.
                let field_type = slice.elem.as_ref().clone();

                Ok((field_type, FieldMode::Slice, field_mut))
            } else {
                let field_type = type_ref.elem.as_ref().clone();

                Ok((field_type, FieldMode::Reference, field_mut))
            }
        }
        Type::Path(type_path) => {
            if let Some(type_ref) = try_parse_option_ref(type_path) {
                // `Option<&T>` reads an `Option<T>` field with `.as_ref()`;
                // `Option<&mut T>` reads it with `.as_mut()`. The read borrows the
                // field with the *inner* reference's mutability, so a `&mut` inner
                // reference likewise requires a `&mut self` receiver.
                require_mut_receiver(type_ref, receiver_mut)?;

                let field_mut = type_ref.mutability;
                let field_type = type_ref.elem.as_ref();

                if field_type == &parse_internal! { str } {
                    // `Option<&str>` is backed by an `Option<String>` field and read
                    // with `.as_deref()` (or `.as_deref_mut()` for `Option<&mut str>`),
                    // mirroring the `&str`/`String` special case for a plain reference.
                    Ok((
                        parse_internal! { Option< String > },
                        FieldMode::OptionStr,
                        field_mut,
                    ))
                } else {
                    Ok((
                        parse_internal! { Option< #field_type > },
                        FieldMode::OptionRef,
                        field_mut,
                    ))
                }
            } else if let Some(field_type) = try_parse_mref(type_path) {
                // `MRef` borrows the field as a shared value, so — unlike a `&mut`
                // reference — its access mode never depends on the receiver.
                Ok((field_type.clone(), FieldMode::MRef, None))
            } else {
                Ok((return_type.clone(), FieldMode::Copy, None))
            }
        }
        // Any other owned type (a tuple, an array, and so on) is read by value and
        // cloned, exactly like an owned path type.
        _ => Ok((return_type.clone(), FieldMode::Copy, None)),
    }
}

/// A mutable field read borrows the whole context exclusively through
/// `get_field_mut`, so a `&mut` reference in the field type — the outer `&mut T` or
/// the inner reference of an `Option<&mut T>` — is only valid under a `&mut self`
/// receiver.
fn require_mut_receiver(type_ref: &TypeReference, receiver_mut: &Option<Mut>) -> syn::Result<()> {
    if type_ref.mutability.is_some() && receiver_mut.is_none() {
        return Err(Error::new(
            type_ref.span(),
            format!(
                "&mut self is required for mutable field reference `{}`",
                type_ref.to_token_stream()
            ),
        ));
    }

    Ok(())
}

fn try_parse_option_ref(type_path: &TypePath) -> Option<&TypeReference> {
    let segment = parse_single_segment_type_path(type_path).ok()?;

    if segment.ident == "Option"
        && let PathArguments::AngleBracketed(args) = &segment.arguments
    {
        let [arg] = Vec::from_iter(args.args.iter()).try_into().ok()?;

        if let GenericArgument::Type(Type::Reference(type_ref)) = arg {
            return Some(type_ref);
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
