//! Acceptable failure: a variant of check_components/missing_dependency.rs where
//! the mistake is not a missing *field* but a missing `#[derive(HasField)]`
//! altogether. `GreetHello` needs `Self: HasName`, and `Person` even has a `name`
//! field — but without the derive, `Person` has *no* `HasField` impls at all, so
//! `HasName` is unsatisfiable and the check fails.
//!
//! This fixture pins the diagnostic that tells this case apart from a single
//! missing field: the `help:` note names `HasField<Symbol!("name")>` as
//! unimplemented for `Person` and points at the `Person` struct, but — unlike
//! missing_dependency.rs, where a derived `age` field supplies a "but trait
//! `HasField<Symbol!(\"age\")>` is implemented for it" landmark — there is no such
//! landmark here, because `Person` implements the trait for no field. The absence
//! of the landmark is the signal that the whole derive is missing and the fix is
//! to add `#[derive(HasField)]`, not to add a field. This is the check doing its
//! job, not a macro defect.
//!
//! See docs/errors/checks/check-trait-failure.md (the "when the derive is missing
//! entirely" variant).

use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasName,
{
    fn greet(&self) {
        let _ = self.name();
    }
}

// The `name` field exists, but without `#[derive(HasField)]` there is no
// `HasField<Symbol!("name")>` impl, so `Person` cannot implement `HasName`.
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

// Fails because `Person` has no `HasField` impls at all, not because it lacks the
// `name` field — the fix is the missing derive.
check_components! {
    Person {
        GreeterComponent,
    }
}

fn main() {}
