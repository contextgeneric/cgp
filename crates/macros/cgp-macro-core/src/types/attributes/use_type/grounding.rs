use std::collections::BTreeMap;

use syn::Ident;
use syn::visit_mut::VisitMut;

use crate::types::attributes::UseTypeAttribute;
use crate::visitors::{SubstituteAbstractTypes, collect_bare_aliases};

/// Resolve every `#[use_type]` spec's groundable positions into fully-qualified
/// form, rejecting an import list whose positions resolve through one another in a
/// cycle.
///
/// Grounding reaches the positions named by
/// [`UseTypeAttribute::groundable_types`] — the context and the trait path's own
/// generic arguments — because an alias left bare in either ends up inside an
/// emitted `<Context as Trait<Args…>>::Assoc` path, resolving to nothing. So
/// `HasTypes.Types, HasScalarType.Scalar in Types` rewrites the second context to
/// `<Self as HasTypes>::Types`, and `HasDbType.Db, HasPoolType<Db>.Pool` projects
/// against `HasPoolType<<Self as HasDbType>::Db>`. A position naming a real generic
/// parameter or `Self` matches no alias and is left untouched.
///
/// **Each spec is grounded against only the specs that are already grounded**, in
/// dependency order, which is what makes one pass per spec sufficient and makes a
/// half-resolved substitution impossible. A single depth-first walk of the
/// dependency graph gives that order and detects a cycle in the same traversal: a
/// spec is grounded once its dependencies return, and an edge back into a spec
/// still on the walk is a cycle. Both properties are therefore structural rather
/// than resting on an earlier validation pass.
///
/// Aliases must already be unique (see `forbid_duplicate_aliases`), since the graph
/// assumes one owning spec per alias.
///
/// The returned specs keep their **source order**, so the supertraits and `where`
/// predicates derived from them read in the order the author wrote the imports.
pub fn ground_specs(specs: &[UseTypeAttribute]) -> syn::Result<Vec<UseTypeAttribute>> {
    // Which spec imports each alias, so a reference can be resolved to the spec it
    // depends on.
    let mut owner_of_alias: BTreeMap<String, usize> = BTreeMap::new();

    for (index, spec) in specs.iter().enumerate() {
        for type_ident in spec.type_idents.iter() {
            owner_of_alias.insert(type_ident.alias_ident().to_string(), index);
        }
    }

    // The dependency edges out of each spec: for every alias its groundable
    // positions reference, the spec owning that alias, together with the referencing
    // token so a rejected cycle can point at what the user wrote.
    //
    // Held outside `Grounding` so that indexing it inside the walk borrows the slice
    // rather than `self`, leaving `self` free to mutate as the walk descends.
    let edges: Vec<Vec<(usize, &Ident)>> = specs
        .iter()
        .map(|spec| {
            spec.groundable_types()
                .flat_map(collect_bare_aliases)
                .filter_map(|ident| Some((*owner_of_alias.get(&ident.to_string())?, ident)))
                .collect()
        })
        .collect();

    let mut grounding = Grounding {
        specs,
        edges: &edges,
        state: vec![VisitState::Unvisited; specs.len()],
        grounded: vec![None; specs.len()],
        labels: Vec::new(),
        path: Vec::new(),
    };

    for index in 0..specs.len() {
        grounding.resolve(index)?;
    }

    Ok(grounding
        .grounded
        .into_iter()
        .map(|spec| spec.expect("every spec is grounded once the walk has visited all of them"))
        .collect())
}

#[derive(Clone, Copy, PartialEq)]
enum VisitState {
    Unvisited,
    /// On the current walk, so an edge back into it closes a cycle.
    InProgress,
    /// Grounded, along with everything it depends on.
    Done,
}

/// The depth-first walk that grounds each spec after its dependencies.
///
/// `path` holds the specs on the current walk and `labels` the referencing tokens
/// that led between them, so a cycle can be reported in the alias names the author
/// wrote rather than as spec indices.
struct Grounding<'a, 'edges> {
    specs: &'a [UseTypeAttribute],
    edges: &'edges [Vec<(usize, &'a Ident)>],
    state: Vec<VisitState>,
    grounded: Vec<Option<UseTypeAttribute>>,
    labels: Vec<&'a Ident>,
    path: Vec<usize>,
}

impl Grounding<'_, '_> {
    fn resolve(&mut self, index: usize) -> syn::Result<()> {
        // `Done` is the ordinary case: the spec and its dependencies are grounded, so
        // there is nothing left to do. `InProgress` cannot arrive here, because the
        // guard at the call site below reports a cycle before recursing — but
        // descending on it anyway would recurse forever, so the test is written to
        // stop on any state but `Unvisited`. A call site that ever skipped the guard
        // then fails as an ungrounded spec at the end of the walk, which is
        // diagnosable, rather than as a stack overflow inside the compiler.
        if self.state[index] != VisitState::Unvisited {
            return Ok(());
        }

        self.state[index] = VisitState::InProgress;
        self.path.push(index);

        for (target, reference) in self.edges[index].iter() {
            if self.state[*target] == VisitState::InProgress {
                return Err(self.cycle_error(*target, reference));
            }

            self.labels.push(reference);
            self.resolve(*target)?;
            self.labels.pop();
        }

        // Every dependency has returned, so all of them are grounded and this spec
        // can be resolved against them.
        self.ground(index);

        self.path.pop();
        self.state[index] = VisitState::Done;

        Ok(())
    }

    /// Substitute one spec's groundable positions against the already-grounded
    /// specs, and record the result.
    fn ground(&mut self, index: usize) {
        // Only grounded specs take part, so a replacement can never carry a bare
        // alias of its own — which is what lets one pass per spec suffice.
        let resolved: Vec<UseTypeAttribute> = self.grounded.iter().flatten().cloned().collect();

        let mut spec = self.specs[index].clone();
        let mut visitor = SubstituteAbstractTypes::new(&resolved);

        for ty in spec.groundable_types_mut() {
            visitor.visit_type_mut(ty);
        }

        self.grounded[index] = Some(spec);
    }

    /// The rejection for an edge back into `target`, labelled `closing`.
    ///
    /// The cycle runs from wherever `target` sits on the current walk back around to
    /// it, so its hops are the labels from that point on plus the closing reference —
    /// which for a self-reference is the single `A` -> `A` hop.
    fn cycle_error(&self, target: usize, closing: &Ident) -> syn::Error {
        let entry = self
            .path
            .iter()
            .position(|index| *index == target)
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
