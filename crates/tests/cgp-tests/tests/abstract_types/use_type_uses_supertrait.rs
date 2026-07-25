//! Re-importing an abstract type with `#[use_type]` even though it already arrives
//! transitively as the supertrait of a trait pulled in by `#[uses]`.
//!
//! `CanCreateFoo` carries `HasFooType` as a supertrait (added by its own
//! `#[use_type(HasFooType.Foo)]`), so `#[uses(CanCreateFoo)]` on `bar` makes
//! `Self: HasFooType` hold transitively. Even so, the recommended form imports
//! `Foo` again with `#[use_type(HasFooType.Foo)]` on `bar`, which lets the body and
//! signature write the bare alias `Foo` instead of a qualified `Self::Foo` that
//! relies on the supertrait being reachable and unambiguous.
//!
//! `bar` forwards to `create_foo`, so this asserts a value flows end to end through
//! the transitively-required, then explicitly re-imported, abstract type.
//!
//! See cgp-knowledge-base/cgp/guides/declaring-dependencies.md and
//! cgp-knowledge-base/cgp/guides/importing-abstract-types.md.

use cgp::prelude::*;

#[cgp_type]
pub trait HasFooType {
    type Foo: Clone;
}

#[cgp_component(FooCreator)]
#[use_type(HasFooType.Foo)]
pub trait CanCreateFoo {
    fn create_foo(&self) -> Foo;
}

#[cgp_impl(new CreateFooFromField)]
#[use_type(HasFooType.Foo)]
impl FooCreator {
    fn create_foo(&self, #[implicit] foo: Foo) -> Foo {
        foo
    }
}

// Recommended: `#[uses(CanCreateFoo)]` supplies `Foo` transitively via the
// `HasFooType` supertrait, but re-importing it with `#[use_type]` lets `bar` write
// the bare `Foo` (in both the return type and the body's binding) rather than a
// qualified `Self::Foo`.
#[cgp_fn]
#[uses(CanCreateFoo)]
#[use_type(HasFooType.Foo)]
fn bar(&self) -> Foo {
    let foo: Foo = self.create_foo();
    foo
}

#[derive(HasField)]
pub struct MyContext {
    pub foo: u64,
}

delegate_and_check_components! {
    MyContext {
        FooTypeProviderComponent: UseType<u64>,
        FooCreatorComponent: CreateFooFromField,
    }
}

#[test]
fn test_bar_forwards_foo() {
    let context = MyContext { foo: 99 };
    assert_eq!(context.bar(), 99);
}
