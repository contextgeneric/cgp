use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit_mut::VisitMut;
use syn::{Ident, Type, WherePredicate};

use crate::functions::parse_internal;
use crate::types::attributes::{UseTypeAttribute, UseTypeIdent};
use crate::visitors::{SubstituteAbstractTypes, collect_bare_aliases};

/// Derive the impl-side `where` predicates a set of `#[use_type]` specs
/// contributes: one `Context: Trait` bound per spec, carrying any type-equality
/// (`= T`) pins as associated-type bindings. The specs' contexts must already be
/// grounded (see [`UseTypeAttributes::grounded_specs`]), so this reads
/// `use_type.context_type` directly rather than re-resolving aliases.
pub fn derive_use_type_predicates(specs: &[UseTypeAttribute]) -> syn::Result<Vec<WherePredicate>> {
    let mut predicates = Vec::new();

    for use_type in specs.iter() {
        let type_equalities = find_type_equalities(use_type, specs);

        let trait_path = &use_type.trait_path;
        let context_type = &use_type.context_type;

        if type_equalities.is_empty() {
            predicates.push(parse_internal! {
                #context_type: #trait_path
            });
        } else {
            // The pins become associated-type bindings *inside* the trait path's
            // own argument list, appended after whatever generic arguments the
            // path already carries. They cannot be a second `<…>` group after the
            // path, because `HasFooType<u8><Foo = u32>` is not a trait bound —
            // a generic trait plus a pin has to render as
            // `HasFooType<u8, Foo = u32>`.
            let mut arguments: Punctuated<TokenStream, Comma> = Punctuated::new();

            for type_arg in trait_path.type_args.args.iter() {
                arguments.push(type_arg.to_token_stream());
            }

            for (alias_ident, equal_target) in type_equalities.into_iter() {
                arguments.push(quote! {
                    #alias_ident = #equal_target
                });
            }

            // Emit the path's head separately from the merged argument list, since
            // `PathWithTypeArgs` renders its own arguments as a trailing group.
            let trait_head = &trait_path.path;

            predicates.push(parse_internal! {
                #context_type: #trait_head < #arguments >
            });
        }
    }

    Ok(predicates)
}

/// Reject two imports that resolve to the same bare identifier or alias, across
/// every spec *and within a single braced list*. A shared alias would make the
/// substitution silently pick the first match and drop the rest, so it is an
/// error regardless of which host macro drives the import.
pub fn forbid_duplicate_aliases(specs: &[UseTypeAttribute]) -> syn::Result<()> {
    let idents: Vec<&UseTypeIdent> = specs
        .iter()
        .flat_map(|spec| spec.type_idents.iter())
        .collect();

    for (index, current) in idents.iter().enumerate() {
        for other in idents.iter().skip(index + 1) {
            if current.alias_ident() == other.alias_ident() {
                return Err(syn::Error::new_spanned(
                    &other.type_ident,
                    "Multiple abstract types cannot share the same identifier or alias",
                ));
            }
        }
    }

    Ok(())
}

/// Reject `#[use_type]` imports whose groundable positions resolve through one
/// another in a cycle, since such an arrangement has no valid grounding order.
///
/// Grounding resolves each spec's context and trait arguments against every other
/// spec, iterating until nothing changes, which lets the imports be written in any
/// order — `HasC.C in B, HasB.B in A, HasA.A` grounds exactly like the
/// front-to-back spelling. A cycle is the one arrangement with no order at all:
/// `#[use_type(HasA.A in B, HasB.B in A)]` asks for each context to be resolved
/// before the other. This check rejects it at macro time with the caret on the
/// alias that closes the loop.
///
/// Detecting it *before* grounding is what makes the check possible, because the
/// obvious after-the-fact test does not work. Grounding does not leave a cyclic
/// alias bare: each pass substitutes against the previous pass's snapshot, in
/// which the other context is still ungrounded, so every pass wraps one more layer
/// and the alias ends up buried at the innermost position of a deep projection
/// (`<<<A as HasB>::B as HasA>::A as HasB>::B`). Asking whether a context is
/// "still a bare alias" afterwards therefore finds nothing. The spec graph, by
/// contrast, states the problem directly.
pub fn forbid_grounding_cycles(specs: &[UseTypeAttribute]) -> syn::Result<()> {
    // Which spec imports each alias. Aliases are unique across all specs, which
    // `forbid_duplicate_aliases` has already established, so one owner per name.
    let mut owner_of_alias: BTreeMap<String, usize> = BTreeMap::new();

    for (index, spec) in specs.iter().enumerate() {
        for type_ident in spec.type_idents.iter() {
            owner_of_alias.insert(type_ident.alias_ident().to_string(), index);
        }
    }

    // The dependency edges out of each spec: for every alias its groundable
    // positions reference, the spec that owns that alias, together with the
    // referencing token so a rejection can point at what the user wrote.
    let edges: Vec<Vec<(usize, &Ident)>> = specs
        .iter()
        .map(|spec| {
            spec.groundable_types()
                .flat_map(collect_bare_aliases)
                .filter_map(|ident| {
                    let target = owner_of_alias.get(&ident.to_string())?;
                    Some((*target, ident))
                })
                .collect()
        })
        .collect();

    // A depth-first search over the spec graph, reporting the first back edge it
    // finds.
    let mut search = CycleSearch {
        edges: &edges,
        state: vec![VisitState::Unvisited; specs.len()],
        path: Vec::new(),
        labels: Vec::new(),
    };

    for start in 0..specs.len() {
        if let Some(error) = search.visit(start) {
            return Err(error);
        }
    }

    Ok(())
}

#[derive(Clone, Copy, PartialEq)]
enum VisitState {
    Unvisited,
    /// On the current search path, so an edge back to it closes a cycle.
    InProgress,
    /// Fully explored and known to reach no cycle.
    Done,
}

/// The depth-first search over the spec dependency graph.
///
/// `path` holds the specs on the current search path and `labels` the referencing
/// tokens that led between them, so a back edge can be rendered as the alias names
/// the user actually wrote rather than as spec indices.
struct CycleSearch<'a, 'edges> {
    edges: &'edges [Vec<(usize, &'a Ident)>],
    state: Vec<VisitState>,
    path: Vec<usize>,
    labels: Vec<&'a Ident>,
}

impl CycleSearch<'_, '_> {
    fn visit(&mut self, spec: usize) -> Option<syn::Error> {
        if self.state[spec] != VisitState::Unvisited {
            return None;
        }

        self.state[spec] = VisitState::InProgress;
        self.path.push(spec);

        for (target, reference) in self.edges[spec].iter() {
            if self.state[*target] == VisitState::InProgress {
                return Some(self.cycle_error(*target, reference));
            }

            self.labels.push(reference);

            if let Some(error) = self.visit(*target) {
                return Some(error);
            }

            self.labels.pop();
        }

        self.path.pop();
        self.state[spec] = VisitState::Done;

        None
    }

    /// The rejection for a back edge into `target`, labelled `closing`.
    ///
    /// The cycle runs from wherever `target` sits on the current path back around
    /// to it, so its hops are the labels from that point on plus the closing
    /// reference — which for a self-reference is the single `A` -> `A` hop.
    fn cycle_error(&self, target: usize, closing: &Ident) -> syn::Error {
        let entry = self
            .path
            .iter()
            .position(|spec| *spec == target)
            .unwrap_or(0);

        let mut hops: Vec<String> = self.labels[entry..]
            .iter()
            .map(|ident| format!("`{ident}`"))
            .collect();

        hops.push(format!("`{closing}`"));
        // Close the loop by restating where it started.
        hops.push(hops[0].clone());

        syn::Error::new_spanned(
            closing,
            format!(
                "cannot ground `#[use_type]` imports: they resolve through one another \
                 in a cycle {}. An `in Context` clause or a trait argument may name another \
                 import's alias only if the resulting chain is acyclic, since a cycle has \
                 no valid grounding order.",
                hops.join(" -> "),
            ),
        )
    }
}

fn find_type_equalities(
    current_spec: &UseTypeAttribute,
    specs: &[UseTypeAttribute],
) -> Vec<(Ident, Type)> {
    current_spec
        .type_idents
        .iter()
        .filter_map(|current_type_ident| find_type_equality(current_type_ident, specs))
        .collect()
}

/// Ground one `= T` pin: rewrite every imported alias appearing *anywhere
/// inside* the pin's right-hand side into its fully-qualified projection, so
/// `{Transaction = Tx<Db>}` grounds its nested `Db` exactly as
/// `{HashedPassword = Password}` grounds a bare one. Substituting through the
/// shared visitor rather than comparing the whole type is what makes the two
/// cases one rule — the right-hand side is an ordinary type, and an alias is
/// resolved wherever it occurs in it.
///
/// The pinned alias itself is excluded from the substitution set, so a
/// degenerate self-pin (`{Foo = Foo}`) stays the unresolved-name error it
/// already was rather than silently becoming a vacuous bound.
fn find_type_equality(
    current_ident: &UseTypeIdent,
    specs: &[UseTypeAttribute],
) -> Option<(Ident, Type)> {
    let mut equal_target = current_ident.equals.clone()?;

    let others = specs_excluding_alias(specs, current_ident.alias_ident());
    SubstituteAbstractTypes::new(&others).visit_type_mut(&mut equal_target);

    Some((current_ident.type_ident.clone(), equal_target))
}

/// The grounded specs with one alias dropped, for substituting inside that
/// alias's own equality pin.
fn specs_excluding_alias(specs: &[UseTypeAttribute], alias: &Ident) -> Vec<UseTypeAttribute> {
    specs
        .iter()
        .map(|spec| {
            let mut spec = spec.clone();
            spec.type_idents
                .retain(|type_ident| type_ident.alias_ident() != alias);
            spec
        })
        .collect()
}
