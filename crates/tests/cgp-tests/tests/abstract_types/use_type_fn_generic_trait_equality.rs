//! `#[use_type]` type-equality on a trait path that already carries generic
//! arguments: `#[use_type(HasFooType<u8>.{Foo = u32})]`.
//!
//! A pin becomes an associated-type binding *inside* the trait path's own argument
//! list, appended after the arguments the path already carries, so the emitted
//! bound reads `Self: HasFooType<u8, Foo = u32>`. It cannot be a second
//! angle-bracketed group after the path, because `HasFooType<u8><Foo = u32>` is not
//! a trait bound at all — an arrangement that used to fail inside the macro with a
//! bare "failed to parse internal tokens" error naming no cause.
//!
//! The second case pins that the merge is per-spec and cumulative: two pins on one
//! generic trait land in the same argument list, while a plain generic import
//! beside them keeps its bare `Self: Trait<Args>` bound.
//!
//! See cgp-knowledge-base/cgp/implementation/asts/attributes/use_type.md and
//! cgp-knowledge-base/cgp/reference/attributes/use_type.md.

use cgp_macro_test_util::snapshot_cgp_fn;

pub trait HasFooType<Tag> {
    type Foo;
}

pub trait HasPairType<Tag> {
    type Left;

    type Right;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasFooType<u8>.{Foo = u32})]
    pub fn get_foo(&self) -> Foo {
        7
    }

    expand_get_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait GetFoo: HasFooType<u8> {
            fn get_foo(&self) -> <Self as HasFooType<u8>>::Foo;
        }
        impl<__Context__> GetFoo for __Context__
        where
            Self: HasFooType<u8, Foo = u32>,
        {
            fn get_foo(&self) -> <Self as HasFooType<u8>>::Foo {
                7
            }
        }
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasPairType<u8>.{Left = u32, Right = u64}, HasFooType<bool>.Foo)]
    pub fn get_pair(&self) -> (Left, Right, Foo) {
        todo!()
    }

    expand_get_pair(output) {
        insta::assert_snapshot!(output, @"
        pub trait GetPair: HasPairType<u8> + HasFooType<bool> {
            fn get_pair(
                &self,
            ) -> (
                <Self as HasPairType<u8>>::Left,
                <Self as HasPairType<u8>>::Right,
                <Self as HasFooType<bool>>::Foo,
            );
        }
        impl<__Context__> GetPair for __Context__
        where
            Self: HasPairType<u8, Left = u32, Right = u64>,
            Self: HasFooType<bool>,
        {
            fn get_pair(
                &self,
            ) -> (
                <Self as HasPairType<u8>>::Left,
                <Self as HasPairType<u8>>::Right,
                <Self as HasFooType<bool>>::Foo,
            ) {
                todo!()
            }
        }
        ")
    }
}

pub trait HasRefType<'a> {
    type Ref;
}

// A pin merged after a *lifetime* argument, which is the ordering-sensitive case:
// Rust requires lifetimes to lead an argument list and associated-type bindings to
// trail it, so appending the binding to the existing arguments has to land on the
// far side of the lifetime.
snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasRefType<'a>.{Ref = &'a str})]
    pub fn get_ref<'a>(&self) -> Ref {
        todo!()
    }

    expand_get_ref(output) {
        insta::assert_snapshot!(output, @"
        pub trait GetRef<'a>: HasRefType<'a> {
            fn get_ref(&self) -> <Self as HasRefType<'a>>::Ref;
        }
        impl<'a, __Context__> GetRef<'a> for __Context__
        where
            Self: HasRefType<'a, Ref = &'a str>,
        {
            fn get_ref(&self) -> <Self as HasRefType<'a>>::Ref {
                todo!()
            }
        }
        ")
    }
}

pub struct App;

impl HasFooType<u8> for App {
    type Foo = u32;
}

#[test]
fn test_pinned_generic_trait_import() {
    // The pin makes the abstract `Foo` concrete, so the value flows out as a `u32`
    // through a bound the macro could not previously even emit.
    let foo: u32 = App.get_foo();
    assert_eq!(foo, 7);
}
