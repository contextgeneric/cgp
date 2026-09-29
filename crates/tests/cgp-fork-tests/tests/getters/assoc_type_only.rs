//! `#[cgp_getter]` on a trait whose only item is an associated type takes the
//! `#[cgp_type]` path: the provider is `{Type}TypeProvider`, and the context
//! fixes the type by wiring `UseType`.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_getter.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_type.md.

use core::fmt::Display;

use cgp_fork::prelude::*;

#[cgp_getter]
pub trait HasLabel {
    type Label: Display;
}

pub struct App;

delegate_components! {
    App {
        LabelTypeProviderComponent: UseType<String>,
    }
}

pub trait CheckLabel: HasLabel<Label = String> {}
impl CheckLabel for App {}
