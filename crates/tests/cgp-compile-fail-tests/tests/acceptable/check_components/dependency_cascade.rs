//! Acceptable failure: one missing field surfaces at every provider that
//! transitively depends on it, so a check of three chained components reports three
//! separate failures for a single root cause. `ProvideFoo` needs the `name` field,
//! `ProvideBar` depends on `CanFoo`, and `ProvideBaz` depends on `CanBar`; wiring all
//! three onto an `App` without a `name` field is accepted, and checking all three
//! forces a cascade of `E0277` blocks — six, not three, because the deeper components
//! also emit intermediate provider-bound failures (`ProvideBar: Bar<App>`,
//! `ProvideFoo: Foo<App>`) beside their `CanUseComponent` failure. Each
//! `CanUseComponent<..>` block does reach the concrete root cause in its `help:` note
//! (`HasField<Symbol!("name")>` not implemented for `App`); the intermediate blocks
//! name only an inner provider trait. Checked top-down (`Baz`, `Bar`, `Foo`), the last
//! block is `Foo`'s clean root-cause block. The count reflects the depth of the
//! dependency graph, not the number of mistakes — fixing the one field collapses the
//! whole cascade. This is the check doing its job, not a macro defect.
//!
//! See docs/errors/checks/verbose-cascade.md.

use cgp::prelude::*;

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_component(Foo)]
pub trait CanFoo {
    fn foo(&self);
}

#[cgp_component(Bar)]
pub trait CanBar {
    fn bar(&self);
}

#[cgp_component(Baz)]
pub trait CanBaz {
    fn baz(&self);
}

#[cgp_impl(new ProvideFoo)]
impl Foo
where
    Self: HasName,
{
    fn foo(&self) {
        let _ = self.name();
    }
}

#[cgp_impl(new ProvideBar)]
#[uses(CanFoo)]
impl Bar {
    fn bar(&self) {
        self.foo();
    }
}

#[cgp_impl(new ProvideBaz)]
#[uses(CanBar)]
impl Baz {
    fn baz(&self) {
        self.bar();
    }
}

#[derive(HasField)]
pub struct App {
    pub age: u8,
}

delegate_components! {
    App {
        FooComponent: ProvideFoo,
        BarComponent: ProvideBar,
        BazComponent: ProvideBaz,
    }
}

// Each checked component fails because `App` lacks the `name` field the innermost
// provider needs, producing one error block per component. Listed top-down, so the
// root-cause block (`Foo`, naming the missing field) is reported last.
check_components! {
    App {
        BazComponent,
        BarComponent,
        FooComponent,
    }
}

fn main() {}
