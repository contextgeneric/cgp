use syn::ext::IdentExt;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    Attribute, Error, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Lit, Token, Visibility,
};

use crate::functions::validate_shape_fields;

/// Parse the body of a `Struct!` into the `syn::Fields` a struct declaration would hold.
///
/// A proc macro never sees the delimiter it was invoked with, so the form is read from the
/// entries: `name: Type` entries make a named body, and bare types make a tuple body. An empty
/// body is `Fields::Unit`. The fields are validated with [`validate_shape_fields`].
pub fn parse_shape_fields(input: ParseStream) -> syn::Result<Fields> {
    parse_entries(input, None)
}

/// Parse the inside of an `Enum!` variant's braces (`named` is true) or parentheses (`named` is
/// false), whose delimiter fixes the form, with the same per-field checks as a `Struct!` body. An
/// empty list keeps its form, so `V {}` and `V()` parse as distinct from the unit `V`, as in a Rust
/// enum, even though all three encode to the same `Nil` payload.
pub fn parse_variant_fields(input: ParseStream, named: bool) -> syn::Result<Fields> {
    parse_entries(input, Some(named))
}

/// Parse comma-separated entries. `required` is the form a variant's delimiter fixes; without it,
/// the first entry decides the form and every later entry must match it.
fn parse_entries(input: ParseStream, required: Option<bool>) -> syn::Result<Fields> {
    let mut named_form = required;
    let mut named = Punctuated::<Field, Comma>::new();
    let mut unnamed = Punctuated::<Field, Comma>::new();

    while !input.is_empty() {
        let entry_is_named = peek_named_entry(input)?;

        match named_form {
            None => named_form = Some(entry_is_named),
            Some(form) if form != entry_is_named => {
                return Err(Error::new(input.span(), form_mismatch_message(required)));
            }
            Some(_) => {}
        }

        if entry_is_named {
            named.push_value(Field::parse_named(input)?);
        } else {
            unnamed.push_value(Field::parse_unnamed(input)?);
        }

        if input.is_empty() {
            break;
        }

        let comma: Comma = input.parse()?;

        if entry_is_named {
            named.push_punct(comma);
        } else {
            unnamed.push_punct(comma);
        }
    }

    let fields = match named_form {
        None => Fields::Unit,
        Some(true) => Fields::Named(FieldsNamed {
            brace_token: Default::default(),
            named,
        }),
        Some(false) => Fields::Unnamed(FieldsUnnamed {
            paren_token: Default::default(),
            unnamed,
        }),
    };

    validate_shape_fields(&fields)?;

    Ok(fields)
}

fn form_mismatch_message(required: Option<bool>) -> &'static str {
    match required {
        None => {
            "cannot mix named and positional fields: write every entry as `name: Type`, or every \
             entry as a bare type"
        }
        Some(true) => "a variant in braces holds named fields: write each entry as `name: Type`",
        Some(false) => {
            "a variant in parentheses holds bare types: write named fields in braces, as in \
             `Variant { name: Type }`"
        }
    }
}

/// Whether the next entry is a `name: Type` field, looking past any attributes and visibility
/// so that those reach their own rejection. A single `:` decides it: `syn` peeks `Token![:]` on
/// the first half of a `::` too, so a path type such as `core::marker::PhantomData<u8>` is ruled
/// out explicitly.
///
/// Two likely mistakes are rejected here, before `syn`'s type parser would report them with a
/// message listing every token a type may start with: a keyword used as a field name, which
/// `peek(Ident)` does not match, and a literal where a type belongs, as when `Struct! { a: 1 }` is
/// read as a struct literal.
fn peek_named_entry(input: ParseStream) -> syn::Result<bool> {
    let fork = input.fork();
    fork.call(Attribute::parse_outer)?;
    fork.parse::<Visibility>()?;

    let is_named = (fork.peek(Ident) || fork.peek(Token![_]))
        && fork.peek2(Token![:])
        && !fork.peek2(Token![::]);

    if is_named {
        fork.call(Ident::parse_any)?;
        fork.parse::<Token![:]>()?;
    } else if fork.peek(Ident::peek_any) && fork.peek2(Token![:]) && !fork.peek2(Token![::]) {
        return Err(keyword_field_error(&fork.call(Ident::parse_any)?));
    }

    if fork.peek(Lit) {
        return Err(Error::new(
            fork.span(),
            "expected a type: a type-level shape lists field types, not values",
        ));
    }

    Ok(is_named)
}

/// A keyword written as a field name. Most keywords have a raw form to suggest; `self`, `Self`,
/// `super`, and `crate` have none and cannot name a field at all.
fn keyword_field_error(keyword: &Ident) -> Error {
    let name = keyword.to_string();

    let message = if matches!(name.as_str(), "self" | "Self" | "super" | "crate") {
        format!("`{name}` cannot be a field name")
    } else {
        format!("`{name}` is a keyword: write the field name as `r#{name}`")
    };

    Error::new(keyword.span(), message)
}
