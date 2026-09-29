use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Comma, Const};
use syn::{Error, Generics, Ident, Lifetime, Token, Type, parse_quote};

use crate::types::ident::{parse_angle_bracketed, to_tokens_angle_bracketed};

/// A single generic parameter that can appear at a *type definition* site,
/// such as each of `'a` and `C` inside `Bar<'a, C>`.
///
/// This is a deliberately restricted version of [`syn::GenericParam`]. Unlike
/// the impl-generics used in `impl` blocks, definition-site parameters in CGP
/// are only ever simple, unconstrained parameters: a bare lifetime, a bare type
/// identifier, or a const parameter. In particular this rejects:
///
/// - trait/lifetime bounds, e.g. `A: Clone` or `'a: 'b`,
/// - defaults, e.g. `A = B` or `const N: usize = 0`,
/// - composite forms, e.g. `(A, B)`.
///
/// This complements (rather than replaces) [`TypeGenerics`], which detects
/// bounds by round-tripping a full `syn::Generics` through `split_for_impl`.
/// Modelling the valid forms directly here is clearer and catches more invalid
/// inputs (such as defaults) up front when *parsing tokens*; see
/// [`TypeGenericParams`] for guidance on which of the two to use.
///
/// [`TypeGenerics`]: crate::types::generics::TypeGenerics
#[derive(Debug, Clone)]
pub enum TypeGenericParam {
    /// A lifetime parameter, e.g. the `'a` in `Bar<'a>`.
    Lifetime(Lifetime),
    /// A type parameter, e.g. the `C` in `Bar<C>`.
    Type(Ident),
    /// A const parameter, e.g. the `const N: usize` in `Bar<const N: usize>`.
    Const(Box<ConstGenericParam>),
}

/// A const generic parameter at a definition site: the `const N: usize` in
/// `Bar<const N: usize>`. Defaults (`= 0`) are deliberately not represented.
#[derive(Debug, Clone)]
pub struct ConstGenericParam {
    pub const_token: Const,
    pub ident: Ident,
    pub colon: Colon,
    pub ty: Type,
}

impl Parse for TypeGenericParam {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Lifetime) {
            let life: Lifetime = input.parse()?;

            if input.peek(Token![:]) {
                return Err(Error::new(
                    life.span(),
                    "lifetime bounds (`'a: 'b`) are not allowed in type generics",
                ));
            }

            return Ok(Self::Lifetime(life));
        }

        if input.peek(Token![const]) {
            let const_token = input.parse()?;
            let ident = input.parse()?;
            let colon = input.parse()?;
            let ty: Type = input.parse()?;

            if input.peek(Token![=]) {
                return Err(Error::new(
                    input.span(),
                    "default const parameters (`const N: T = ...`) are not allowed in type generics",
                ));
            }

            return Ok(Self::Const(Box::new(ConstGenericParam {
                const_token,
                ident,
                colon,
                ty,
            })));
        }

        let ident: Ident = input.parse()?;

        if input.peek(Token![:]) {
            return Err(Error::new(
                ident.span(),
                "trait bounds (`A: Clone`) are not allowed in type generics",
            ));
        }

        if input.peek(Token![=]) {
            return Err(Error::new(
                ident.span(),
                "default type parameters (`A = B`) are not allowed in type generics",
            ));
        }

        Ok(Self::Type(ident))
    }
}

impl ToTokens for TypeGenericParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Lifetime(life) => life.to_tokens(tokens),
            Self::Type(ident) => ident.to_tokens(tokens),
            Self::Const(param) => {
                param.const_token.to_tokens(tokens);
                param.ident.to_tokens(tokens);
                param.colon.to_tokens(tokens);
                param.ty.to_tokens(tokens);
            }
        }
    }
}

/// The angle-bracketed parameter list at a type definition site, e.g. the
/// `<'a, C>` in `Bar<'a, C>`.
///
/// # `TypeGenericParams` vs [`TypeGenerics`]
///
/// Both model a definition-site generic list, but they are different tools:
///
/// - Reach for `TypeGenericParams` when **parsing tokens** where you want the
///   restrictions enforced strictly and the parameters classified by kind. It
///   is a hand-written parser that rejects bounds and defaults up front and
///   exposes each parameter as a [`TypeGenericParam`] variant.
/// - Reach for [`TypeGenerics`] when adapting an **already-parsed
///   [`syn::Generics`]** (e.g. off an `ItemTrait`/`ItemStruct`). It is a thin
///   newtype that `Deref`s to `syn::Generics`, so `split_for_impl()` and the
///   usual `syn` manipulation are available, and its `TryFrom<&Generics>`
///   normalizes through `split_for_impl` (which, notably, collapses a
///   `const N: T` parameter down to a bare type-like `N`).
///
/// They are intentionally not merged: the normalization behavior above is
/// load-bearing for some callers, so a faithful conversion into the strict
/// `TypeGenericParam` model would change behavior around const generics.
///
/// [`TypeGenerics`]: crate::types::generics::TypeGenerics
///
/// Both the absence of angle brackets and an explicit empty `<>` are
/// represented as an empty [`Punctuated`]. An empty list renders as nothing,
/// so a parsed `<>` round-trips back to no angle brackets.
#[derive(Debug, Clone, Default)]
pub struct TypeGenericParams {
    pub params: Punctuated<TypeGenericParam, Comma>,
}

impl TypeGenericParams {
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    /// Lower these parameters into a plain [`syn::Generics`]. This is handy for
    /// downstream code that needs to feed the parameters into constructs (such
    /// as struct definitions) that are expressed in terms of `syn::Generics`.
    pub fn to_generics(&self) -> Generics {
        parse_quote!( #self )
    }
}

impl Parse for TypeGenericParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            params: parse_angle_bracketed(input)?,
        })
    }
}

impl ToTokens for TypeGenericParams {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        to_tokens_angle_bracketed(&self.params, tokens);
    }
}
