use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, Gt, Lt};
use syn::{Error, Expr, ExprBlock, ExprLit, GenericArgument, Lifetime, Lit, Token, Type};

/// A single generic argument that can appear in a *type expression* position,
/// such as each of `'a`, `A`, `(A, B)`, and `Bar<A>` inside
/// `Foo<'a, A, (A, B), Bar<A>>`.
///
/// This is a deliberately restricted version of [`syn::GenericArgument`]. The
/// `syn` type additionally accepts associated type bindings (`Item = T`),
/// associated const bindings (`N = 1`), and associated type bounds
/// (`Item: Clone`), none of which are valid in the plain type-argument
/// positions that CGP cares about. By modelling only the three valid forms, we
/// reject inputs like `Foo<A, B = C>` at parse time.
#[derive(Debug, Clone)]
pub enum TypeArg {
    /// A lifetime argument, e.g. the `'a` in `Foo<'a>`.
    Lifetime(Lifetime),
    /// A type argument, e.g. the `A`, `(A, B)`, or `Bar<A>` in `Foo<A>`.
    Type(Type),
    /// A const argument written as a literal or a braced block, e.g. the `3`
    /// in `Foo<3>` or the `{ N }` in `Foo<{ N }>`.
    ///
    /// Note that, just like `syn`, a bare identifier const argument (such as
    /// the `N` in `Foo<N>`) is syntactically indistinguishable from a type and
    /// is therefore parsed as [`TypeArg::Type`].
    Const(Expr),
}

impl TypeArg {
    /// Convert a [`syn::GenericArgument`] into a [`TypeArg`], rejecting the
    /// associated-binding and bound forms that are not valid in type-argument
    /// positions.
    ///
    /// This is useful when post-processing a value that `syn` has already
    /// parsed into a [`syn::Path`] (see [`PathWithTypeArgs`]).
    ///
    /// [`PathWithTypeArgs`]: crate::types::ident::PathWithTypeArgs
    pub fn from_generic_argument(arg: &GenericArgument) -> syn::Result<Self> {
        match arg {
            GenericArgument::Lifetime(life) => Ok(Self::Lifetime(life.clone())),
            GenericArgument::Type(ty) => Ok(Self::Type(ty.clone())),
            GenericArgument::Const(expr) => Ok(Self::Const(expr.clone())),
            GenericArgument::AssocType(_) | GenericArgument::AssocConst(_) => {
                Err(Error::new_spanned(
                    arg,
                    "associated bindings (`Name = ...`) are not allowed in type arguments",
                ))
            }
            GenericArgument::Constraint(_) => Err(Error::new_spanned(
                arg,
                "associated type bounds (`Name: ...`) are not allowed in type arguments",
            )),
            _ => Err(Error::new_spanned(arg, "unsupported generic argument")),
        }
    }
}

impl Parse for TypeArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Lifetime) {
            return Ok(Self::Lifetime(input.parse()?));
        }

        // Const arguments are only recognized when written as a literal or a
        // braced block, mirroring `syn::GenericArgument`. A bare identifier is
        // parsed as a `Type` instead.
        if input.peek(Lit) {
            let lit: Lit = input.parse()?;
            return Ok(Self::Const(Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit,
            })));
        }

        // A braced block must be parsed as an `ExprBlock` rather than a general
        // `Expr`, because `Expr::parse` would greedily treat a following `>` as
        // a comparison operator (e.g. parsing `{ N } >` as `{ N } > ...`).
        if input.peek(Brace) {
            let block: ExprBlock = input.parse()?;
            return Ok(Self::Const(Expr::Block(block)));
        }

        let ty: Type = input.parse()?;

        // After a complete type, the only valid continuation in an argument
        // list is `,` or `>`. An `=` or `:` here indicates an associated
        // binding or bound, which `syn::AngleBracketedGenericArguments` would
        // silently accept but which is invalid in this position.
        if input.peek(Token![=]) {
            return Err(Error::new(
                input.span(),
                "associated bindings (`Name = ...`) are not allowed in type arguments",
            ));
        }

        if input.peek(Token![:]) {
            return Err(Error::new(
                input.span(),
                "associated type bounds (`Name: ...`) are not allowed in type arguments",
            ));
        }

        Ok(Self::Type(ty))
    }
}

impl ToTokens for TypeArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Lifetime(life) => life.to_tokens(tokens),
            Self::Type(ty) => ty.to_tokens(tokens),
            Self::Const(expr) => expr.to_tokens(tokens),
        }
    }
}

/// The optional angle-bracketed argument list that follows an identifier or
/// path in a type expression, e.g. the `<'a, A, Bar<A>>` in `Foo<'a, A, Bar<A>>`.
///
/// `None` represents the absence of any angle brackets (the bare `Foo` case),
/// whereas `Some(empty)` represents an explicit empty `Foo<>`. This mirrors the
/// behaviour of the existing `GenericArguments` type so the two can be used
/// interchangeably during migration.
#[derive(Debug, Clone, Default)]
pub struct TypeArgs {
    pub args: Option<Punctuated<TypeArg, Comma>>,
}

impl TypeArgs {
    /// Get a mutable reference to the underlying argument list, inserting an
    /// empty `<>` list if none is present yet. This matches the
    /// `GenericArguments::make_args` API to ease migration.
    pub fn make_args(&mut self) -> &mut Punctuated<TypeArg, Comma> {
        self.args.get_or_insert_with(Punctuated::new)
    }

    pub fn is_empty(&self) -> bool {
        match &self.args {
            Some(args) => args.is_empty(),
            None => true,
        }
    }
}

impl Parse for TypeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if !input.peek(Lt) {
            return Ok(Self { args: None });
        }

        let _: Lt = input.parse()?;

        let mut args = Punctuated::new();

        while !input.peek(Gt) {
            args.push_value(input.parse()?);

            if input.peek(Gt) {
                break;
            }

            args.push_punct(input.parse()?);
        }

        let _: Gt = input.parse()?;

        Ok(Self { args: Some(args) })
    }
}

impl ToTokens for TypeArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(args) = &self.args {
            tokens.extend(quote! { < #args > });
        }
    }
}
