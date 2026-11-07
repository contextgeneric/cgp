//! Every macro that declares a public item documents it.
//!
//! The module denies `missing_docs`, so this file fails to compile if a macro
//! stops emitting a doc line for an item it generates. It lives in the library
//! crate rather than an integration target because `missing_docs` only fires on
//! items exported from a library; a test binary exports nothing. The snapshot
//! modules pin the text of the generated docs, and this pins their presence.

#![deny(missing_docs)]
// A doc comment on a macro invocation is consumed by the macro, which emits it on
// the generated items, so rustdoc never sees it here.
#![allow(unused_doc_comments)]

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::prelude::*;

/// A component, whose marker struct the macro documents.
#[cgp_component(Greeter)]
pub trait CanGreet {
    /// Greets.
    fn greet(&self) -> String;
}

/// A provider, whose struct the macro documents.
#[cgp_impl(new GreetHello)]
impl Greeter {
    fn greet(&self) -> String {
        "Hello".to_owned()
    }
}

/// A generic provider, whose struct and `PhantomData` field the macro documents.
#[cgp_impl(new GreetWith<Name>)]
impl<Name> Greeter
where
    Name: Send + Sync,
{
    fn greet(&self) -> String {
        "Hello".to_owned()
    }
}

/// A function-style trait, whose trait and method the macro documents.
#[cgp_fn]
pub fn greet_fn(&self, #[implicit] name: &str) -> String {
    name.to_owned()
}

/// An abstract type, whose marker struct the macro documents.
#[cgp_type]
pub trait HasProbeType {
    /// The probe type.
    type Probe;
}

/// A getter, whose component the macro documents.
#[cgp_getter]
pub trait HasProbeGetter {
    /// Reads the probe.
    fn probe(&self) -> &str;
}

/// An auto getter, whose blanket impl the macro documents.
#[cgp_auto_getter]
pub trait HasProbeField {
    /// Reads the field.
    fn probe_field(&self) -> &str;
}

/// A computer, whose provider struct the macro documents.
#[cgp_computer]
pub fn probe_computer(input: u32) -> u32 {
    input
}

/// A producer, whose provider struct the macro documents.
#[cgp_producer]
pub fn probe_producer() -> u32 {
    1
}

/// A dispatch trait, whose provider struct the macro documents.
#[cgp_auto_dispatch]
pub trait CanProbeDispatch {
    /// Dispatches.
    fn dispatch(&self) -> u32;
}

/// A namespace, whose struct, trait, and associated type the macro documents.
cgp_namespace! {
    new ProbeNamespace: DefaultNamespace {
        @cgp.core.error.ErrorTypeProviderComponent: UseType<String>,
    }
}

/// A wiring table, whose struct the macro documents.
delegate_components! {
    new ProbeTable {
        GreeterComponent: GreetHello,
    }
}

/// A record, whose partial builder the derive documents.
#[derive(CgpData)]
pub struct ProbeRecord {
    /// A field.
    pub field: u32,
}

/// A variant, whose partial extractor the derive documents.
#[derive(CgpData)]
pub enum ProbeVariant {
    /// A variant.
    A(u32),
}
