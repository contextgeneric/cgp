use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Ident, Path, PathArguments, Type, parse_quote, parse2};

use crate::traits::ToType;
use crate::types::ident::{IdentWithTypeArgs, TypeArgs};

/// A full Rust path followed by an optional type-expression argument list, e.g.
/// `Foo`, `Foo<A, B>`, `path::to::Foo`, or `path::to::Bar<(A, B), B>`.
///
/// This generalizes [`IdentWithTypeArgs`] from a single identifier head to a
/// full [`syn::Path`] head. The motivation is that `syn::Path` keeps the final
/// generic arguments buried inside the last [`syn::PathSegment`], which is
/// awkward to read and rewrite. This type lifts those arguments out into a
/// separate [`TypeArgs`] field while keeping the remaining path in `path`,
/// applying the same restrictions as [`TypeArg`](crate::types::ident::TypeArg)
/// (no associated bindings or bounds).
///
/// Only the final segment may carry generic arguments. Intermediate generics
/// (e.g. `path::to<X>::Foo`) and parenthesized arguments (e.g. `Fn(A) -> B`)
/// are rejected.
#[derive(Debug, Clone)]
pub struct PathWithTypeArgs {
    /// The full path with the final segment's arguments stripped, e.g.
    /// `path::to::Foo` for an input of `path::to::Foo<A, B>`.
    pub path: Path,
    /// The arguments lifted out of the final path segment, e.g. `<A, B>`.
    pub type_args: TypeArgs,
}

impl PathWithTypeArgs {
    /// The identifier of the final path segment, e.g. `Foo` in
    /// `path::to::Foo<A, B>`.
    pub fn ident(&self) -> &Ident {
        &self
            .path
            .segments
            .last()
            .expect("PathWithTypeArgs always wraps a non-empty syn::Path")
            .ident
    }
}

impl Parse for PathWithTypeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut path: Path = input.parse()?;

        let last_index = path.segments.len() - 1;

        // Generic arguments are only meaningful on the final segment for our
        // use cases. Reject them on intermediate segments.
        for (index, segment) in path.segments.iter().enumerate() {
            if index != last_index && !segment.arguments.is_none() {
                return Err(Error::new_spanned(
                    segment,
                    "generic arguments are only allowed on the final path segment",
                ));
            }
        }

        let last_segment = path.segments.last_mut().unwrap();

        let type_args = match &last_segment.arguments {
            PathArguments::None => TypeArgs::default(),
            PathArguments::AngleBracketed(arguments) => {
                // Reject turbofish (`Foo::<A>`); only the type-position form
                // `Foo<A>` is accepted, matching `IdentWithTypeArgs`.
                if arguments.colon2_token.is_some() {
                    return Err(Error::new_spanned(
                        arguments,
                        "turbofish arguments (`Foo::<A>`) are not allowed; use `Foo<A>`",
                    ));
                }

                // Re-parse the already-parsed `<...>` through `TypeArgs` so the
                // argument-form restrictions (no associated bindings or bounds)
                // live in exactly one place — `TypeArg`'s own parser — rather
                // than being duplicated here against `syn::GenericArgument`.
                // With the turbofish ruled out above, `arguments` re-emits as a
                // plain `< .. >`, which is exactly what `TypeArgs` expects.
                parse2::<TypeArgs>(arguments.to_token_stream())?
            }
            PathArguments::Parenthesized(arguments) => {
                return Err(Error::new_spanned(
                    arguments,
                    "parenthesized generic arguments (`Fn(A) -> B`) are not allowed",
                ));
            }
        };

        // Keep `path` free of the final arguments so that `ToTokens` can
        // reconstruct the original input as `path` followed by `type_args`.
        last_segment.arguments = PathArguments::None;

        Ok(Self { path, type_args })
    }
}

impl ToTokens for PathWithTypeArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.type_args.to_tokens(tokens);
    }
}

impl From<Ident> for PathWithTypeArgs {
    fn from(ident: Ident) -> Self {
        Self {
            path: Path::from(ident),
            type_args: TypeArgs::default(),
        }
    }
}

impl From<IdentWithTypeArgs> for PathWithTypeArgs {
    fn from(value: IdentWithTypeArgs) -> Self {
        Self {
            path: Path::from(value.ident),
            type_args: value.type_args,
        }
    }
}

impl ToType for PathWithTypeArgs {
    fn to_type(&self) -> Type {
        parse_quote!(#self)
    }
}

impl From<PathWithTypeArgs> for Type {
    fn from(value: PathWithTypeArgs) -> Self {
        value.to_type()
    }
}
