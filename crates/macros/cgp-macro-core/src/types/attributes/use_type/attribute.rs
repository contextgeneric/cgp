use syn::parse::{Parse, ParseStream};
use syn::token::{Brace, Comma, Dot, In};
use syn::{Ident, Type};

use crate::parse_internal;
use crate::types::attributes::UseTypeIdent;
use crate::types::ident::{PathWithTypeArgs, TypeArg};

/// One `#[use_type(...)]` import spec: the owning trait path, one or more
/// associated types to import from it, and a rewrite target (`Self`, or a named
/// type set by a trailing `in Context`).
#[derive(Clone)]
pub struct UseTypeAttribute {
    pub context_type: Type,
    pub trait_path: PathWithTypeArgs,
    pub type_idents: Vec<UseTypeIdent>,
}

impl UseTypeAttribute {
    /// The type positions in this spec that *grounding* resolves: the context the
    /// imported types are projected against, and the generic arguments of the
    /// owning trait path.
    ///
    /// Both positions end up inside the emitted `<Context as Trait<Args…>>::Assoc`
    /// path, so an alias left bare in either is an identifier that resolves to
    /// nothing. Naming the set once — rather than at each of the three places that
    /// walk it — is what keeps the grounding pass, the emitted bounds, and the
    /// cycle check in agreement about what grounding reaches.
    pub fn groundable_types(&self) -> impl Iterator<Item = &Type> {
        core::iter::once(&self.context_type).chain(
            self.trait_path
                .type_args
                .args
                .iter()
                .filter_map(|arg| match arg {
                    TypeArg::Type(ty) => Some(ty),
                    // A lifetime or const argument can never name an abstract type.
                    TypeArg::Lifetime(_) | TypeArg::Const(_) => None,
                }),
        )
    }

    /// [`Self::groundable_types`] by mutable reference, for the grounding pass.
    pub fn groundable_types_mut(&mut self) -> impl Iterator<Item = &mut Type> {
        core::iter::once(&mut self.context_type).chain(
            self.trait_path
                .type_args
                .args
                .iter_mut()
                .filter_map(|arg| match arg {
                    TypeArg::Type(ty) => Some(ty),
                    TypeArg::Lifetime(_) | TypeArg::Const(_) => None,
                }),
        )
    }

    pub fn replace_ident(&self, ident: &Ident) -> Option<Ident> {
        for type_ident in &self.type_idents {
            if type_ident.alias_ident() == ident {
                let mut new_ident = type_ident.type_ident.clone();
                new_ident.set_span(ident.span());
                return Some(new_ident);
            }
        }

        None
    }
}

impl Parse for UseTypeAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // A `.` (not `::`) separates the trait from the associated type. This
        // keeps the trait unambiguous even when it is a full path such as
        // `foo::bar::HasScalarType`: `::` stays inside the path, and the trailing
        // `.` marks where the associated type begins.
        let trait_path: PathWithTypeArgs = input.parse()?;

        let _: Dot = input.parse()?;

        let type_idents: Vec<UseTypeIdent> = if input.peek(Brace) {
            let content;
            syn::braced!(content in input);
            content
                .parse_terminated(UseTypeIdent::parse, Comma)?
                .into_iter()
                .collect()
        } else {
            vec![input.parse()?]
        };

        // An optional `in Context` suffix sets the rewrite target to a named
        // type; `in` is a reserved keyword, so it can never be confused with a
        // trait, type, or associated-type name and reads as a clean delimiter.
        // Without the suffix, the target defaults to `Self`.
        let context_type: Type = if input.peek(In) {
            let _: In = input.parse()?;
            let context: PathWithTypeArgs = input.parse()?;
            context.into()
        } else {
            parse_internal! { Self }
        };

        Ok(Self {
            context_type,
            trait_path,
            type_idents,
        })
    }
}
