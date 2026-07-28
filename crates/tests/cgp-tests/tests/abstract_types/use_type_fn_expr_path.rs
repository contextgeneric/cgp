//! `#[use_type]` resolving an alias that *qualifies* an expression path:
//! `Transaction::begin_from(pool)` inside a `#[cgp_fn]` body.
//!
//! The substitution rewrites an imported alias in every type position, including a
//! `let` annotation inside the body, so leaving it unresolved as the qualifier of an
//! associated function or associated const was an inconsistency rather than a
//! boundary. Both are rewritten here into the qualified-type form
//! `<<Self as Trait>::Assoc>::item`, and the `let` annotation in the same body pins
//! that the type-position rewrite still works alongside it.
//!
//! The boundary the rewrite keeps is arity: a path of two or more segments can only
//! name an associated item of the type, while a bare single-segment path in
//! expression position is a *value* — a unit struct or an enum variant — which an
//! abstract type can never be, so it is left alone. `unit_struct_untouched` pins
//! that half: `Marker` in type position is the imported alias while `Marker` as an
//! expression stays the unit struct.
//!
//! See cgp-knowledge-base/cgp/reference/attributes/use_type.md and
//! cgp-knowledge-base/cgp/implementation/asts/attributes/use_type.md.

use cgp_macro_test_util::snapshot_cgp_fn;

pub trait Database: Sized {
    type Row;
}

pub struct Pool<Db>(pub core::marker::PhantomData<Db>);

pub struct Tx<Db>(pub core::marker::PhantomData<Db>);

pub struct Postgres;

impl Database for Postgres {
    type Row = String;
}

pub trait CanBeginFrom<Db> {
    const LABEL: &'static str;

    fn begin_from(pool: &Pool<Db>) -> Self;
}

impl CanBeginFrom<Postgres> for Tx<Postgres> {
    const LABEL: &'static str = "pg";

    fn begin_from(_pool: &Pool<Postgres>) -> Self {
        Tx(core::marker::PhantomData)
    }
}

pub trait HasDbType {
    type Db: Database;
}

pub trait HasTransactionType {
    type Transaction;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasDbType.Db, HasTransactionType.Transaction)]
    pub fn begin_transaction(&self, #[implicit] db: &Pool<Db>) -> Transaction
    where
        Transaction: CanBeginFrom<Db>,
    {
        let started: Transaction = Transaction::begin_from(db);
        started
    }

    expand_begin_transaction(output) {
        insta::assert_snapshot!(output, @"
        pub trait BeginTransaction: HasDbType + HasTransactionType {
            fn begin_transaction(&self) -> <Self as HasTransactionType>::Transaction;
        }
        impl<__Context__> BeginTransaction for __Context__
        where
            <Self as HasTransactionType>::Transaction: CanBeginFrom<<Self as HasDbType>::Db>,
            Self: HasField<
                Symbol<2, Chars<'d', Chars<'b', Nil>>>,
                Value = Pool<<Self as HasDbType>::Db>,
            >,
            Self: HasDbType,
            Self: HasTransactionType,
        {
            fn begin_transaction(&self) -> <Self as HasTransactionType>::Transaction {
                let db: &Pool<<Self as HasDbType>::Db> = self
                    .get_field(
                        ::core::marker::PhantomData::<Symbol<2, Chars<'d', Chars<'b', Nil>>>>,
                    );
                let started: <Self as HasTransactionType>::Transaction = <<Self as HasTransactionType>::Transaction>::begin_from(
                    db,
                );
                started
            }
        }
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasDbType.Db, HasTransactionType.Transaction)]
    pub fn transaction_label(&self) -> &'static str
    where
        Transaction: CanBeginFrom<Db>,
    {
        Transaction::LABEL
    }

    expand_transaction_label(output) {
        insta::assert_snapshot!(output, @"
        pub trait TransactionLabel: HasDbType + HasTransactionType {
            fn transaction_label(&self) -> &'static str;
        }
        impl<__Context__> TransactionLabel for __Context__
        where
            <Self as HasTransactionType>::Transaction: CanBeginFrom<<Self as HasDbType>::Db>,
            Self: HasDbType,
            Self: HasTransactionType,
        {
            fn transaction_label(&self) -> &'static str {
                <<Self as HasTransactionType>::Transaction>::LABEL
            }
        }
        ")
    }
}

pub struct Marker;

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasDbType.{Db as Marker})]
    pub fn unit_struct_untouched(&self, #[implicit] db: &Pool<Marker>) -> Marker2 {
        let _ = db;
        Marker.into()
    }

    expand_unit_struct_untouched(output) {
        insta::assert_snapshot!(output, @"
        pub trait UnitStructUntouched: HasDbType {
            fn unit_struct_untouched(&self) -> Marker2;
        }
        impl<__Context__> UnitStructUntouched for __Context__
        where
            Self: HasField<
                Symbol<2, Chars<'d', Chars<'b', Nil>>>,
                Value = Pool<<Self as HasDbType>::Db>,
            >,
            Self: HasDbType,
        {
            fn unit_struct_untouched(&self) -> Marker2 {
                let db: &Pool<<Self as HasDbType>::Db> = self
                    .get_field(
                        ::core::marker::PhantomData::<Symbol<2, Chars<'d', Chars<'b', Nil>>>>,
                    );
                let _ = db;
                Marker.into()
            }
        }
        ")
    }
}

pub struct Marker2;

impl From<Marker> for Marker2 {
    fn from(_: Marker) -> Self {
        Marker2
    }
}
