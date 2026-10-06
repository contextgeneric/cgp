use proc_macro2::TokenStream;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, Paren};
use syn::{Attribute, Error, Fields, Ident, Token, Type, Variant, Visibility};

use crate::functions::{reject_non_empty_attributes, validate_shape_fields};
use crate::types::cgp_data::variants_to_sum_type;

/// The `Enum!` macro: the body of an enum, held as the `syn::Variant` list the enum
/// declaration would have.
///
/// As with [`StructType`](super::StructType), holding the derive's input form lets
/// [`eval`](Self::eval) run the `#[derive(HasFields)]` encoder unchanged.
pub struct EnumType {
    pub variants: Punctuated<Variant, Comma>,
}

impl EnumType {
    /// Encode the variants as the derive does: a sum of `Field<Symbol!("Variant"), Payload>`
    /// entries ending in `Void`, each payload encoded by the `Struct!` rules.
    pub fn eval(&self) -> syn::Result<Type> {
        variants_to_sum_type(&self.variants, &TokenStream::new())
    }
}

impl Parse for EnumType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variants = Punctuated::new();
        let mut seen = Vec::new();

        while !input.is_empty() {
            let variant = parse_variant(input)?;

            let name = variant.ident.unraw().to_string();
            if seen.contains(&name) {
                return Err(Error::new(
                    variant.ident.span(),
                    format!("duplicate variant `{name}`"),
                ));
            }
            seen.push(name);

            variants.push_value(variant);

            if input.is_empty() {
                break;
            }

            variants.push_punct(input.parse()?);
        }

        Ok(Self { variants })
    }
}

/// Parse one variant. `syn`'s own `Variant` parser silently discards a visibility and accepts
/// a discriminant, so the variant is read here, rejecting both, and its fields are parsed with
/// `syn`'s struct-body parsers. Unlike the `Struct!` body, a variant's delimiter is visible, so
/// braces hold named fields and parentheses positional ones, as in a Rust enum.
fn parse_variant(input: ParseStream) -> syn::Result<Variant> {
    let attrs = input.call(Attribute::parse_outer)?;
    reject_non_empty_attributes(&attrs)?;

    if input.peek(Token![pub]) {
        let visibility: Visibility = input.parse()?;
        return Err(Error::new_spanned(
            visibility,
            "an `Enum!` variant has no visibility; remove it",
        ));
    }

    let ident: Ident = input.parse()?;

    let fields = if input.peek(Brace) {
        Fields::Named(input.parse()?)
    } else if input.peek(Paren) {
        Fields::Unnamed(input.parse()?)
    } else {
        Fields::Unit
    };

    validate_shape_fields(&fields)?;

    if input.peek(Token![=]) {
        return Err(Error::new(
            input.span(),
            "an `Enum!` variant has no discriminant; remove the `= …`",
        ));
    }

    Ok(Variant {
        attrs: Vec::new(),
        ident,
        fields,
        discriminant: None,
    })
}
