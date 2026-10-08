use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Field, FieldMutability, Fields, FieldsUnnamed, Type, Variant, Visibility};

use crate::macro_core::exports::Nil;
use crate::macro_core::parse_internal;

/// The payload of a variant the variant derives accept: either the single
/// unnamed field of a newtype variant, or `Nil` for a variant with no fields.
pub enum VariantPayload<'a> {
    /// A newtype variant `V(T)`, whose payload is `T`.
    Single(&'a Type),
    /// A variant with no fields, written `V`, `V()`, or `V {}`, whose payload is
    /// `Nil`, the empty product `#[derive(HasFields)]` already gives it.
    Empty,
}

impl VariantPayload<'_> {
    /// The payload type: the field's type, or `Nil`.
    pub fn payload_type(&self) -> syn::Result<Type> {
        match self {
            Self::Single(field_type) => Ok((*field_type).clone()),
            Self::Empty => Ok(parse_internal! { #Nil }),
        }
    }

    /// A pattern matching the variant on the original enum, binding the payload
    /// to `value`. An empty variant is matched by `{ .. }`, which matches all
    /// three of its forms.
    pub fn match_pattern(&self, variant: &Variant) -> TokenStream {
        let variant_ident = &variant.ident;

        match self {
            Self::Single(_) => quote! { Self :: #variant_ident ( value ) },
            Self::Empty => quote! { Self :: #variant_ident { .. } },
        }
    }

    /// An expression building the variant on the original enum from a payload
    /// bound to `value`. An empty variant is built by `{}`, which builds all
    /// three of its forms and ignores the payload.
    pub fn constructor(&self, variant: &Variant) -> TokenStream {
        let variant_ident = &variant.ident;

        match self {
            Self::Single(_) => quote! { Self :: #variant_ident ( value ) },
            Self::Empty => quote! { Self :: #variant_ident {} },
        }
    }

    /// The binding for a payload the variant is built from: `value` for a newtype
    /// variant, and `_` for an empty one, which ignores its `Nil`.
    pub fn binding(&self) -> TokenStream {
        match self {
            Self::Single(_) => quote! { value },
            Self::Empty => quote! { _ },
        }
    }

    /// The payload the owned extractor holds for a matched variant.
    pub fn owned_value(&self) -> TokenStream {
        match self {
            Self::Single(_) => quote! { value },
            Self::Empty => quote! { #Nil },
        }
    }

    /// The payload the shared borrowed extractor holds: a promoted `&Nil` for an
    /// empty variant, since it has no field to borrow.
    pub fn ref_value(&self) -> TokenStream {
        match self {
            Self::Single(_) => quote! { value },
            Self::Empty => quote! { &#Nil },
        }
    }

    /// The payload the mutable borrowed extractor holds. An empty variant has no
    /// field to borrow and Rust does not promote `&mut Nil`, so the payload is a
    /// leaked `Box<Nil>`, which never allocates because `Nil` is zero-sized.
    ///
    /// `Box` is written bare and resolved where the derive is used, the one name
    /// a CGP expansion takes from the caller's scope rather than from `cgp_fork`,
    /// which links no `alloc`. It is spanned at the variant, so a `no_std` crate
    /// without `Box` in scope sees the error on the empty variant.
    pub fn mut_value(&self, variant: &Variant) -> TokenStream {
        match self {
            Self::Single(_) => quote! { value },
            Self::Empty => {
                let span = variant.ident.span();
                let boxed = quote_spanned! { span => Box };

                quote! { #boxed::leak(#boxed::new(#Nil)) }
            }
        }
    }
}

/// Return a variant's payload, or a spanned error if the variant has several
/// fields or named fields, which the variant derives cannot carry as one payload.
pub fn get_variant_payload(variant: &Variant) -> syn::Result<VariantPayload<'_>> {
    match &variant.fields {
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            if let Some(field) = fields.unnamed.first() {
                return Ok(VariantPayload::Single(&field.ty));
            }
        }
        Fields::Unnamed(fields) if fields.unnamed.is_empty() => {
            return Ok(VariantPayload::Empty);
        }
        Fields::Named(fields) if fields.named.is_empty() => {
            return Ok(VariantPayload::Empty);
        }
        Fields::Unit => return Ok(VariantPayload::Empty),
        _ => {}
    }

    Err(syn::Error::new(
        variant.span(),
        "Expected variant to contain exactly one unnamed field, or no fields",
    ))
}

/// Return a variant's payload type: the newtype field's type, or `Nil` for a
/// variant with no fields.
pub fn get_variant_type(variant: &Variant) -> syn::Result<Type> {
    get_variant_payload(variant)?.payload_type()
}

/// Wrap a type as a single-unnamed-field variant body, used to re-shape a
/// partial enum's variants around their `MapType`-wrapped payloads.
pub fn type_to_variant_fields(type_: &Type) -> Fields {
    Fields::Unnamed(FieldsUnnamed {
        unnamed: Punctuated::from_iter([Field {
            attrs: Vec::new(),
            ident: None,
            vis: Visibility::Inherited,
            ty: type_.clone(),
            colon_token: None,
            mutability: FieldMutability::None,
        }]),
        paren_token: Default::default(),
    })
}
