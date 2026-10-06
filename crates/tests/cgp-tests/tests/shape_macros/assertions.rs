//! Type-equality assertions shared by the `shape_macros` tests.

use core::any::{TypeId, type_name};

/// Implemented by a type only for itself, which is what lets
/// [`assert_same_type`] compare two types through a trait bound.
pub trait SameTypeAs<T: ?Sized> {}

impl<T: ?Sized> SameTypeAs<T> for T {}

/// Compiles only if `A` and `B` are the same type.
///
/// A mismatch is a compile error naming both types, as the unsatisfied bound
/// `A: SameTypeAs<B>`.
pub fn assert_same_type<A, B>()
where
    A: ?Sized + SameTypeAs<B>,
    B: ?Sized,
{
}

/// Panics if `A` and `B` are the same type.
///
/// The compiler cannot assert that two types differ, so this compares their
/// `TypeId`s at run time, which needs both types to be `'static`.
#[track_caller]
pub fn assert_type_ne<A, B>()
where
    A: ?Sized + 'static,
    B: ?Sized + 'static,
{
    assert_ne!(
        TypeId::of::<A>(),
        TypeId::of::<B>(),
        "expected `{}` and `{}` to be different types",
        type_name::<A>(),
        type_name::<B>(),
    );
}
