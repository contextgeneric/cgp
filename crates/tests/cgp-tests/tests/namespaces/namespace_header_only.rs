//! A `cgp_namespace!` that ends at its header, with no brace pair after it.
//!
//! When a namespace has no entries of its own the body may be omitted:
//! `cgp_namespace! { new HeaderOnlyNamespace }` emits just the marker struct and
//! the lookup trait, and `cgp_namespace! { new HeaderOnlyExtendedNamespace: DefaultNamespace }`
//! adds the inheritance impl, exactly as the same headers followed by `{ }` do.
//! Both snapshots belong to this concept, which owns `cgp_namespace!`. `App` then
//! joins the header-only inheriting namespace and wires the inherited error type
//! and raiser components under their full `@cgp.core.error.*` paths, and the
//! `CheckApp` bundle asserts each wired component resolves, confirming the
//! braceless definition is a usable namespace and not just a parse. That
//! `delegate_components!` is written plainly, since its `namespace` form is
//! pinned in `prefix_default_namespace`.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_namespace.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_namespace.md.

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::RaiseFrom;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_namespace;

snapshot_cgp_namespace! {
    cgp_namespace! {
        new HeaderOnlyNamespace
    }

    expand_header_only_namespace(output) {
        insta::assert_snapshot!(output, @"
        pub struct __HeaderOnlyNamespaceComponents;
        pub trait HeaderOnlyNamespace<__Table__> {
            type Delegate;
        }
        ")
    }
}

snapshot_cgp_namespace! {
    cgp_namespace! {
        new HeaderOnlyExtendedNamespace: DefaultNamespace
    }

    expand_header_only_extended_namespace(output) {
        insta::assert_snapshot!(output, @"
        pub struct __HeaderOnlyExtendedNamespaceComponents;
        pub trait HeaderOnlyExtendedNamespace<__Table__> {
            type Delegate;
        }
        impl<__Table__, __Key__, __Value__> HeaderOnlyExtendedNamespace<__Table__> for __Key__
        where
            __Key__: DefaultNamespace<__HeaderOnlyExtendedNamespaceComponents>,
            __Key__: DefaultNamespace<__Table__, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        ")
    }
}

pub struct App;

delegate_components! {
    App {
        namespace HeaderOnlyExtendedNamespace;

        @cgp.core.error.ErrorTypeProviderComponent:
            UseType<String>,
        @cgp.core.error.ErrorRaiserComponent.{&'static str, String}:
            RaiseFrom,
    }
}

pub trait CheckApp: HasErrorType + CanRaiseError<&'static str> + CanRaiseError<String> {}

impl CheckApp for App {}
