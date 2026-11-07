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
    parse_field_type_with(return_type, receiver_mut, false)
}

/// Like [`parse_field_type`], but `borrow_str` decides how a `&str` is read. An
/// `#[implicit]` argument sets it, so the field may be any `AsRef<str>` and needs no
/// allocator; a getter leaves it off, because its `HasField` bound is also used by
/// the `WithProvider` adapter, which expects the field type itself.
pub fn parse_field_type_with(
    return_type: &Type,
    receiver_mut: &Option<Mut>,
    borrow_str: bool,
) -> syn::Result<(Type, FieldMode, Option<Mut>)> {
    match &return_type {
        Type::Reference(type_ref) => {
            require_mut_receiver(type_ref, receiver_mut)?;

            let field_mut = type_ref.mutability;

            if type_ref.elem.as_ref() == &parse_internal! { str } {
                if borrow_str {
                    // A `&str` reads any `AsRef<str>` field with `.as_ref()`, so a
                    // `String`, a `Cow<str>`, or a `Box<str>` field all work, and
                    // none of them needs an allocator.
                    let field_type: Type = parse_internal! { str };

                    Ok((field_type, FieldMode::Str, field_mut))
                } else {
                    // A getter's `HasField` bound is also used by the `WithProvider`
                    // adapter, which expects the field type itself, so it keeps the
                    // `String` field.
                    let field_type: Type = parse_internal! { String };

                    Ok((field_type, FieldMode::Str, field_mut))
                }
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
                reject_unsized_option_ref(type_path, type_ref)?;

                // `Option<&T>` reads an `Option<T>` field with `.as_ref()`;
                // `Option<&mut T>` reads it with `.as_mut()`. The read borrows the
                // field with the *inner* reference's mutability, so a `&mut` inner
                // reference likewise requires a `&mut self` receiver.
                require_mut_receiver(type_ref, receiver_mut)?;

                let field_mut = type_ref.mutability;
                let field_type = type_ref.elem.as_ref();

                if field_type == &parse_internal! { str } {
                    if borrow_str {
                        // An `#[implicit]` argument reads any `Option<U>` whose `U`
                        // implements `AsRef<str>` through `OptionAsRef`, so a
                        // `String`, a `Cow<str>`, or a `Box<str>` field all work and
                        // none needs an allocator.
                        Ok((parse_internal! { str }, FieldMode::OptionStr, field_mut))
                    } else {
                        // A getter keeps the `Option<String>` field, because its bound
                        // is also used by the `WithProvider` adapter.
                        Ok((
                            parse_internal! { Option< String > },
                            FieldMode::OptionStr,
                            field_mut,
                        ))
                    }
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

/// `Option<&T>` reads an `Option<T>` field, and an `Option` holds only a sized value, so an optional
/// reference to a slice or a trait object has no field to read. Rejected here, naming the sized
/// container to read instead, rather than lowered to a bound over an unsized `Option`. The message
/// quotes the reference as written and names the referent inside each container, so
/// `Option<&[u8]>` points to an `Option<Vec<u8>>` field and `Option<&(dyn Debug + Send)>` to an
/// `Option<Box<dyn Debug + Send>>` field.
fn reject_unsized_option_ref(type_path: &TypePath, type_ref: &TypeReference) -> syn::Result<()> {
    let mut referent = type_ref.elem.as_ref();
    while let Type::Paren(paren) = referent {
        referent = paren.elem.as_ref();
    }

    let (kind, container) = match referent {
        Type::Slice(slice) => ("a slice", format!("Vec<{}>", type_text(&slice.elem))),
        Type::TraitObject(_) => ("a trait object", format!("Box<{}>", type_text(referent))),
        _ => return Ok(()),
    };

    let reference = if type_ref.mutability.is_some() {
        "&mut "
    } else {
        "&"
    };

    Err(Error::new_spanned(
        type_path,
        format!(
            "`Option<{}>` would read an `Option<{}>` field, which cannot exist because {kind} is unsized; read an `Option<{container}>` field as `Option<{reference}{container}>` instead",
            type_text(type_ref),
            type_text(referent),
        ),
    ))
}

/// A type as the reader writes it, from its tokens: the spaces the token printer puts around `::`,
/// `<`, `>`, `,`, `;`, brackets, parentheses, and after `&` removed, so `core :: fmt :: Debug` reads
/// `core::fmt::Debug`, `& mut [u8 ; 4]` reads `&mut [u8; 4]`, and `Fn (u8)` reads `Fn(u8)`.
fn type_text(ty: &impl ToTokens) -> String {
    let mut text = ty.to_token_stream().to_string();
    for (spaced, tight) in [
        (" :: ", "::"),
        (" < ", "<"),
        ("< ", "<"),
        (" <", "<"),
        (" >", ">"),
        (" ,", ","),
        (" ;", ";"),
        ("[ ", "["),
        (" ]", "]"),
        ("( ", "("),
        (" )", ")"),
        ("& ", "&"),
    ] {
        text = text.replace(spaced, tight);
    }
    join_argument_lists(&text)
}

/// Removes the space before a parenthesized argument list, which follows a name, as in `Fn (u8)`.
/// A parenthesized type keeps its space, so `-> (u8, u16)`, `&'a (dyn Debug)`, and
/// `&mut (dyn Debug)` are unchanged: after a lifetime or the keyword `mut`, `dyn`, or `impl`, the
/// group is not an argument list.
fn join_argument_lists(text: &str) -> String {
    let mut joined = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(" (") {
        let before = &rest[..at];
        let prefix = before.trim_end_matches(|c: char| c.is_alphanumeric() || c == '_');
        let word = &before[prefix.len()..];
        let is_argument_list =
            !word.is_empty() && !prefix.ends_with('\'') && !matches!(word, "mut" | "dyn" | "impl");

        joined.push_str(before);
        if !is_argument_list {
            joined.push(' ');
        }
        joined.push('(');
        rest = &rest[at + 2..];
    }
    joined.push_str(rest);
    joined
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
