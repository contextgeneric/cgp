//! `#[use_type]` grounding an imported alias that appears in a *trait path's own
//! generic arguments*: `#[use_type(HasDbType.Db, HasPoolType<Db>.Pool)]`.
//!
//! A trait argument ends up inside the emitted `<Context as Trait<Args…>>::Assoc`
//! path exactly as the context does, so an alias left bare in it names an
//! identifier that resolves to nothing. Grounding therefore resolves both
//! positions: `HasPoolType<Db>` becomes `HasPoolType<<Self as HasDbType>::Db>`, in
//! the rewritten signature and in the appended supertrait alike.
//!
//! This is the same rule that already grounds an `in Context` clause and a pin's
//! right-hand side, so the trait-argument position was an inconsistency rather than
//! a boundary — the alias resolved in two of the three positions it can occupy.
//!
//! The second case pins that grounding a trait argument composes with pinning the
//! type imported from it, since the merged argument list has to carry both the
//! grounded argument and the binding.
//!
//! See cgp-knowledge-base/cgp/implementation/asts/attributes/use_type.md and
//! cgp-knowledge-base/cgp/reference/attributes/use_type.md.

use cgp_macro_test_util::snapshot_cgp_fn;

pub trait HasDbType {
    type Db;
}

pub trait HasPoolType<Db> {
    type Pool;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasDbType.Db, HasPoolType<Db>.Pool)]
    pub fn get_pool(&self) -> Pool {
        todo!()
    }

    expand_get_pool(output) {
        insta::assert_snapshot!(output, @"
        pub trait GetPool: HasDbType + HasPoolType<<Self as HasDbType>::Db> {
            fn get_pool(&self) -> <Self as HasPoolType<<Self as HasDbType>::Db>>::Pool;
        }
        impl<__Context__> GetPool for __Context__
        where
            Self: HasDbType,
            Self: HasPoolType<<Self as HasDbType>::Db>,
        {
            fn get_pool(&self) -> <Self as HasPoolType<<Self as HasDbType>::Db>>::Pool {
                todo!()
            }
        }
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasDbType.Db, HasPoolType<Db>.{Pool = u32})]
    pub fn get_pinned_pool(&self) -> Pool {
        11
    }

    expand_get_pinned_pool(output) {
        insta::assert_snapshot!(output, @"
        pub trait GetPinnedPool: HasDbType + HasPoolType<<Self as HasDbType>::Db> {
            fn get_pinned_pool(&self) -> <Self as HasPoolType<<Self as HasDbType>::Db>>::Pool;
        }
        impl<__Context__> GetPinnedPool for __Context__
        where
            Self: HasDbType,
            Self: HasPoolType<<Self as HasDbType>::Db, Pool = u32>,
        {
            fn get_pinned_pool(&self) -> <Self as HasPoolType<<Self as HasDbType>::Db>>::Pool {
                11
            }
        }
        ")
    }
}

pub trait HasHandleType<Pool> {
    type Handle;
}

// A three-hop chain threaded entirely through trait arguments rather than through
// `in Context` clauses, so each hop's argument must be grounded before the next can
// project against it. This is the trait-argument analogue of the context chain in
// `use_type_fn_deep_foreign`, and it exercises the same grounding fixpoint over the
// new position.
snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasDbType.Db, HasPoolType<Db>.Pool, HasHandleType<Pool>.Handle)]
    pub fn get_handle(&self) -> Handle {
        todo!()
    }

    expand_get_handle(output) {
        insta::assert_snapshot!(output, @"
        pub trait GetHandle: HasDbType + HasPoolType<
                <Self as HasDbType>::Db,
            > + HasHandleType<<Self as HasPoolType<<Self as HasDbType>::Db>>::Pool> {
            fn get_handle(
                &self,
            ) -> <Self as HasHandleType<
                <Self as HasPoolType<<Self as HasDbType>::Db>>::Pool,
            >>::Handle;
        }
        impl<__Context__> GetHandle for __Context__
        where
            Self: HasDbType,
            Self: HasPoolType<<Self as HasDbType>::Db>,
            Self: HasHandleType<<Self as HasPoolType<<Self as HasDbType>::Db>>::Pool>,
        {
            fn get_handle(
                &self,
            ) -> <Self as HasHandleType<
                <Self as HasPoolType<<Self as HasDbType>::Db>>::Pool,
            >>::Handle {
                todo!()
            }
        }
        ")
    }
}

pub struct Postgres;

pub struct App;

impl HasDbType for App {
    type Db = Postgres;
}

impl HasPoolType<Postgres> for App {
    type Pool = u32;
}

#[test]
fn test_grounded_trait_argument() {
    // `HasPoolType<<App as HasDbType>::Db>` resolves to `HasPoolType<Postgres>`, so
    // both functions type-check against the one impl and return its `u32` pool.
    assert_eq!(App.get_pinned_pool(), 11);
}
