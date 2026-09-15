//! `Self::Assoc` inside a `#[cgp_impl]` body survives the rewrite when the block
//! declares `Assoc` itself.
//!
//! `#[cgp_impl]` rewrites `Self` into the context, which would break a component
//! whose provider supplies an associated type: `Self::Output` in a signature or a
//! body means the *provider's* own `type Output`, not something the context has.
//! The rewrite therefore collects the associated types the block declares and
//! skips any `Self::` path whose first segment names one of them, so the emitted
//! provider impl keeps `Self::Output` — where `Self` is now the provider struct
//! that declares it. Every other `Self` in the same block, including the `Self`
//! of an inherited abstract type, is still rewritten to the context.
//!
//! An associated *const* is deliberately not covered by the skip, since only
//! `ImplItem::Type` items are collected; `#[cgp_impl]`'s Known issues records
//! that asymmetry and the `<Provider as Trait<Self>>::CONST` form that works
//! around it.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_impl.md (Behavior and corner cases)
//! and cgp-knowledge-base/cgp/reference/macros/cgp_impl.md.

use cgp::prelude::*;

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_component(Labeller)]
pub trait CanLabel {
    type Label;

    fn label(&self) -> Self::Label;
}

#[cgp_impl(new LabelWithName)]
impl Labeller
where
    Self: HasName,
{
    // The provider decides the associated type. Both the `Self::Label` in the
    // return position and the one in the body name *this* item, so neither may be
    // rewritten to the context — while `self.name()` must be.
    type Label = String;

    fn label(&self) -> Self::Label {
        let label: Self::Label = format!("<{}>", self.name());
        label
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        LabellerComponent: LabelWithName,
    }
}

#[test]
fn test_self_local_assoc_type() {
    let person = Person {
        name: "World".to_owned(),
    };

    assert_eq!(person.label(), "<World>");
}
