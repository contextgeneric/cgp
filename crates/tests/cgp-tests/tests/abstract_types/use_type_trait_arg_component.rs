//! A grounded trait argument across the idiomatic host pair: a `#[cgp_component]`
//! and a `#[cgp_impl]` provider that both import through
//! `#[use_type(HasDbType.Db, HasPoolType<Db>.Pool)]`, wired onto a real context.
//!
//! The two hosts consume the grounded spec through different paths — a component
//! turns a `Self` import into a *supertrait*, while an impl turns it into a `where`
//! predicate that also carries any pin — so this exercises both halves of the
//! transform on one shared abstract type. The provider additionally pins the
//! imported type (`{Pool = u32}`), which the impl-side path merges into the already
//! grounded trait argument as `HasPoolType<<Self as HasDbType>::Db, Pool = u32>`.
//!
//! The `#[cgp_fn]` counterparts, which pin the emitted tokens as snapshots, are
//! `use_type_fn_trait_arg_alias` and `use_type_fn_generic_trait_equality`; this file
//! pins that the same grounding survives component wiring and dispatches at run
//! time.
//!
//! See cgp-knowledge-base/cgp/implementation/asts/attributes/use_type.md and
//! cgp-knowledge-base/cgp/reference/attributes/use_type.md.

use cgp::prelude::*;

#[cgp_type]
pub trait HasDbType {
    type Db;
}

#[cgp_type]
pub trait HasPoolType<Db> {
    type Pool;
}

#[cgp_component(PoolSizeReader)]
#[use_type(HasDbType.Db, HasPoolType<Db>.Pool)]
pub trait CanReadPoolSize {
    fn read_pool_size(&self) -> Pool;
}

// The provider pins the abstract pool type to `u32` while still projecting it
// against the grounded `HasPoolType<<Self as HasDbType>::Db>`.
#[cgp_impl(new ReadPoolSize)]
#[use_type(HasDbType.Db, HasPoolType<Db>.{Pool = u32})]
impl PoolSizeReader {
    fn read_pool_size(&self) -> Pool {
        42
    }
}

pub struct Postgres;

pub struct App;

delegate_components! {
    App {
        DbTypeProviderComponent: UseType<Postgres>,
        PoolTypeProviderComponent: UseType<u32>,
        PoolSizeReaderComponent: ReadPoolSize,
    }
}

check_components! {
    App {
        PoolSizeReaderComponent,
    }
}

#[test]
fn test_grounded_trait_argument_wiring() {
    // The context binds `Db` to `Postgres` and `Pool` to `u32`, which is what makes
    // the provider's grounded-and-pinned bound hold.
    assert_eq!(App.read_pool_size(), 42);
}
