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
    let mut named_form = None;
    let mut named = Punctuated::<Field, Comma>::new();
    let mut unnamed = Punctuated::<Field, Comma>::new();

    while !input.is_empty() {
        let entry_is_named = peek_named_entry(input)?;

        match named_form {
            None => named_form = Some(entry_is_named),
            Some(form) if form != entry_is_named => {
                return Err(Error::new(
                    input.span(),
                    "cannot mix named and positional fields: write every entry as \
                     `name: Type`, or every entry as a bare type",
                ));
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

/// Whether the next entry is a `name: Type` field, looking past any attributes and visibility
/// so that those reach their own rejection. A single `:` decides it: `syn` peeks `Token![:]` on
/// the first half of a `::` too, so a path type such as `core::marker::PhantomData<u8>` is ruled
/// out explicitly. Also rejects a literal where a type belongs, the likely mistake of reading
/// `Struct! { a: 1 }` as a struct literal.
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
    }

    if fork.peek(Lit) {
        return Err(Error::new(
            fork.span(),
            "expected a type: a type-level shape lists field types, not values",
        ));
    }

    Ok(is_named)
}
