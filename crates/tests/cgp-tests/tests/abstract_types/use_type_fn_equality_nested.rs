//! `#[use_type]` type-equality whose right-hand side *contains* an imported alias
//! rather than *being* one: `#[use_type(HasDbType.Db, HasTransactionType.{Transaction = Tx<Db>})]`.
//!
//! The cross-trait sibling `use_type_fn_equality_cross_trait` pins the case where
//! the pin's right-hand side *is* another alias (`{Bar as Baz = Foo}`). This file
//! pins the general rule that supersedes it: the right-hand side is an ordinary
//! type, and an imported alias is grounded wherever it occurs inside it. Both
//! shapes an alias can hide in are covered — nested in a generic argument list
//! (`Tx<Db>`) and inside a qualified path (`<Db as Database>::Transaction`) — each
//! emitting a bound whose right-hand side names `<Self as HasDbType>::Db` rather
//! than a bare `Db` that resolves to nothing.
//!
//! The pinned alias itself is excluded from its own substitution, so a degenerate
//! self-pin stays the unresolved-name error it already was.
//!
//! See cgp-knowledge-base/cgp/reference/attributes/use_type.md and
//! cgp-knowledge-base/cgp/implementation/asts/attributes/use_type.md.

use cgp_macro_test_util::snapshot_cgp_fn;

pub trait Database: Sized {
    type Transaction;
}

pub struct Tx<Db>(pub core::marker::PhantomData<Db>);

pub trait HasDbType {
    type Db: Database;
}

pub trait HasTransactionType {
    type Transaction;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasDbType.Db, HasTransactionType.{Transaction = Tx<Db>})]
    pub fn begin_nested(&self) -> Transaction {
        todo!()
    }

    expand_begin_nested(output) {
        insta::assert_snapshot!(output, @"
        pub trait BeginNested: HasDbType + HasTransactionType {
            fn begin_nested(&self) -> <Self as HasTransactionType>::Transaction;
        }
        impl<__Context__> BeginNested for __Context__
        where
            Self: HasDbType,
            Self: HasTransactionType<Transaction = Tx<<Self as HasDbType>::Db>>,
        {
            fn begin_nested(&self) -> <Self as HasTransactionType>::Transaction {
                todo!()
            }
        }
        ")
    }
}

// The same nested pin with the imports written in the other order, so the pin's
// right-hand side names an alias declared *after* it. Resolution grounds each spec
// against the specs it depends on rather than the ones preceding it, so the emitted
// bound is identical; only the order the bounds are listed in follows the source.
snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasTransactionType.{Transaction = Tx<Db>}, HasDbType.Db)]
    pub fn begin_nested_reversed(&self) -> Transaction {
        todo!()
    }

    expand_begin_nested_reversed(output) {
        insta::assert_snapshot!(output, @"
        pub trait BeginNestedReversed: HasTransactionType + HasDbType {
            fn begin_nested_reversed(&self) -> <Self as HasTransactionType>::Transaction;
        }
        impl<__Context__> BeginNestedReversed for __Context__
        where
            Self: HasTransactionType<Transaction = Tx<<Self as HasDbType>::Db>>,
            Self: HasDbType,
        {
            fn begin_nested_reversed(&self) -> <Self as HasTransactionType>::Transaction {
                todo!()
            }
        }
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasDbType.Db, HasTransactionType.{Transaction = <Db as Database>::Transaction})]
    pub fn begin_projected(&self) -> Transaction {
        todo!()
    }

    expand_begin_projected(output) {
        insta::assert_snapshot!(output, @"
        pub trait BeginProjected: HasDbType + HasTransactionType {
            fn begin_projected(&self) -> <Self as HasTransactionType>::Transaction;
        }
        impl<__Context__> BeginProjected for __Context__
        where
            Self: HasDbType,
            Self: HasTransactionType<
                Transaction = <<Self as HasDbType>::Db as Database>::Transaction,
            >,
        {
            fn begin_projected(&self) -> <Self as HasTransactionType>::Transaction {
                todo!()
            }
        }
        ")
    }
}
