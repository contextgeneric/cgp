//! Several `#[use_type]` attributes *stacked* on one item, rather than one
//! attribute carrying a comma-separated list.
//!
//! A host collector appends each attribute's specs to one list, so stacking is
//! expected to behave exactly like the combined form. The combined form is
//! recommended for readability, but the equivalence is a documented promise and is
//! pinned here — including for the case where it could plausibly break, a pin whose
//! right-hand side names an alias imported by a *different attribute*
//! (`#[use_type(HasFooType.{Foo = Vec<Bar>})]` above `#[use_type(HasBarType.Bar)]`).
//!
//! Both stacking orders are pinned, because nothing about resolution may depend on
//! where a spec was written: it grounds each spec against the specs it depends on,
//! not against the ones that precede it. The only thing source order decides is the
//! order the emitted bounds are *listed* in, which the two snapshots below show.
//!
//! See cgp-knowledge-base/cgp/implementation/asts/attributes/use_type.md and
//! cgp-knowledge-base/cgp/reference/attributes/use_type.md.

use cgp_macro_test_util::snapshot_cgp_fn;

pub trait HasBarType {
    type Bar;
}

pub trait HasFooType {
    type Foo;
}

// The pinning attribute comes first, so its right-hand side names an alias the
// attribute *below* it imports.
snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasFooType.{Foo = Vec<Bar>})]
    #[use_type(HasBarType.Bar)]
    pub fn get_foo(&self) -> Foo {
        Vec::new()
    }

    expand_get_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait GetFoo: HasFooType + HasBarType {
            fn get_foo(&self) -> <Self as HasFooType>::Foo;
        }
        impl<__Context__> GetFoo for __Context__
        where
            Self: HasFooType<Foo = Vec<<Self as HasBarType>::Bar>>,
            Self: HasBarType,
        {
            fn get_foo(&self) -> <Self as HasFooType>::Foo {
                Vec::new()
            }
        }
        ")
    }
}

// The same two imports stacked the other way round.
snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasBarType.Bar)]
    #[use_type(HasFooType.{Foo = Vec<Bar>})]
    pub fn get_foo_reversed(&self) -> Foo {
        Vec::new()
    }

    expand_get_foo_reversed(output) {
        insta::assert_snapshot!(output, @"
        pub trait GetFooReversed: HasBarType + HasFooType {
            fn get_foo_reversed(&self) -> <Self as HasFooType>::Foo;
        }
        impl<__Context__> GetFooReversed for __Context__
        where
            Self: HasBarType,
            Self: HasFooType<Foo = Vec<<Self as HasBarType>::Bar>>,
        {
            fn get_foo_reversed(&self) -> <Self as HasFooType>::Foo {
                Vec::new()
            }
        }
        ")
    }
}

pub struct App;

impl HasBarType for App {
    type Bar = u32;
}

impl HasFooType for App {
    type Foo = Vec<u32>;
}

#[test]
fn test_stacked_attributes_resolve() {
    // `Foo` is pinned to `Vec<<App as HasBarType>::Bar>`, so both functions return
    // the context's `Vec<u32>` whichever order the attributes were stacked in.
    let foo: Vec<u32> = App.get_foo();
    let reversed: Vec<u32> = App.get_foo_reversed();

    assert!(foo.is_empty());
    assert!(reversed.is_empty());
}
