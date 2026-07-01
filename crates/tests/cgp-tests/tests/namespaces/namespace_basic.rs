//! The basic `cgp_namespace!` form and wiring a context through a namespace.
//!
//! `cgp_namespace! { new MyNamespace { .. } }` builds a namespace trait/table, an
//! entry maps a component onto a `RedirectLookup` path, and a component attaches
//! itself to the namespace with `#[prefix(@MyBarComponent in MyNamespace)]`. A
//! context then wires through `namespace MyNamespace;` + `@`-path entries. The
//! `cgp_namespace!`, `#[prefix]`-component, and namespace `delegate_components!`
//! snapshots are the canonical golden output this concept owns; the plain `Foo`
//! component and the two provider impls are incidental scaffolding, written with
//! the plain macros (their expansion is pinned in `basic_delegation`).
//!
//! See docs/reference/macros/cgp_namespace.md and
//! docs/reference/macros/delegate_components.md.

use cgp::prelude::*;
use cgp_macro_test_util::{
    snapshot_cgp_component, snapshot_cgp_namespace, snapshot_delegate_components,
};

// Incidental: a plain component used only to be wired through the namespace.
#[cgp_component(FooProvider)]
pub trait Foo {
    fn foo(&self);
}

snapshot_cgp_namespace! {
    cgp_namespace! {
        new MyNamespace {
            FooProviderComponent =>
                @MyFooComponent,
        }
    }

    expand_basic_my_namespace(output) {
        insta::assert_snapshot!(output, @"
        pub struct __MyNamespaceComponents;
        pub trait MyNamespace<__Table__> {
            type Delegate;
        }
        impl<__Table__> MyNamespace<__Table__> for FooProviderComponent {
            type Delegate = RedirectLookup<__Table__, PathCons<MyFooComponent, Nil>>;
        }
        ")
    }
}

snapshot_cgp_component! {
    #[cgp_component(BarProvider)]
    #[prefix(@MyBarComponent in MyNamespace)]
    pub trait Bar {
        fn bar(&self);
    }

    expand_basic_bar(output) {
        insta::assert_snapshot!(output, @"
        pub trait Bar {
            fn bar(&self);
        }
        impl<__Context__> Bar for __Context__
        where
            __Context__: BarProvider<__Context__>,
        {
            fn bar(&self) {
                __Context__::bar(self)
            }
        }
        pub trait BarProvider<
            __Context__,
        >: IsProviderFor<BarProviderComponent, __Context__, ()> {
            fn bar(__context__: &__Context__);
        }
        impl<__Provider__, __Context__> BarProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<BarProviderComponent>
                + IsProviderFor<BarProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                BarProviderComponent,
            >>::Delegate: BarProvider<__Context__>,
        {
            fn bar(__context__: &__Context__) {
                <__Provider__ as DelegateComponent<
                    BarProviderComponent,
                >>::Delegate::bar(__context__)
            }
        }
        pub struct BarProviderComponent;
        impl<__Context__> BarProvider<__Context__> for UseContext
        where
            __Context__: Bar,
        {
            fn bar(__context__: &__Context__) {
                __Context__::bar(__context__)
            }
        }
        impl<__Context__> IsProviderFor<BarProviderComponent, __Context__, ()> for UseContext
        where
            __Context__: Bar,
        {}
        impl<__Context__, __Components__, __Path__> BarProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: BarProvider<__Context__>,
        {
            fn bar(__context__: &__Context__) {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::bar(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<BarProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<BarProviderComponent, __Context__, ()>
                + BarProvider<__Context__>,
        {}
        impl<__Components__> MyNamespace<__Components__> for BarProviderComponent {
            type Delegate = RedirectLookup<
                __Components__,
                PathCons<MyBarComponent, PathCons<BarProviderComponent, Nil>>,
            >;
        }
        ")
    }
}

pub struct MyFooComponent;
pub struct MyBarComponent;

// Incidental: plain providers wired to the namespace below.
#[cgp_impl(new DummyFoo)]
impl FooProvider {
    fn foo(&self) {}
}

#[cgp_impl(new DummyBar)]
impl BarProvider {
    fn bar(&self) {}
}

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            namespace MyNamespace;

            @MyFooComponent:
                DummyFoo,
            @MyBarComponent:
                DummyBar,
        }
    }

    expand_basic_app(output) {
        insta::assert_snapshot!(output, @"
        impl<__Key__, __Value__> DelegateComponent<__Key__> for App
        where
            __Key__: MyNamespace<App, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        impl<
            __Key__,
            __Value__,
            __Context__,
            __Params__,
        > IsProviderFor<__Key__, __Context__, __Params__> for App
        where
            __Key__: MyNamespace<App, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        impl<__Wildcard__> DelegateComponent<PathCons<MyFooComponent, __Wildcard__>> for App {
            type Delegate = DummyFoo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<PathCons<MyFooComponent, __Wildcard__>, __Context__, __Params__> for App
        where
            DummyFoo: IsProviderFor<
                PathCons<MyFooComponent, __Wildcard__>,
                __Context__,
                __Params__,
            >,
        {}
        impl<__Wildcard__> DelegateComponent<PathCons<MyBarComponent, __Wildcard__>> for App {
            type Delegate = DummyBar;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<PathCons<MyBarComponent, __Wildcard__>, __Context__, __Params__> for App
        where
            DummyBar: IsProviderFor<
                PathCons<MyBarComponent, __Wildcard__>,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

check_components! {
    App {
        FooProviderComponent,
        BarProviderComponent,
    }
}
