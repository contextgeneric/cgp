//! One component attached to two different namespaces at once.
//!
//! Two namespaces (`MyNamespace` with a type path `@MyApp.MyFooComponent`, and
//! `OtherNamespace` with a symbol path `@my_app.MyFooComponent`) are defined with
//! `cgp_namespace!`, and a single `Bar` component attaches to both by stacking two
//! `#[prefix(.. in ..)]` attributes — so its snapshot carries one namespace impl
//! per attribute. Two contexts then wire the same providers through the two
//! namespaces. The `cgp_namespace!`, `#[prefix]`-component, and namespace
//! `delegate_components!` snapshots are kept; the plain `Foo` component and the
//! provider impls are incidental scaffolding.
//!
//! See docs/reference/macros/cgp_namespace.md and
//! docs/reference/macros/delegate_components.md.

use cgp::prelude::*;
use cgp_macro_test_util::{
    snapshot_cgp_component, snapshot_cgp_namespace, snapshot_delegate_components,
};

pub struct MyApp;

// Incidental: a plain component used only to be wired through the namespaces.
#[cgp_component(FooProvider)]
pub trait Foo {
    fn foo(&self);
}

snapshot_cgp_namespace! {
    cgp_namespace! {
        new MyNamespace {
            FooProviderComponent =>
                @MyApp.MyFooComponent,
        }
    }

    expand_multi_ns_my_namespace(output) {
        insta::assert_snapshot!(output, @"
        pub struct __MyNamespaceComponents;
        pub trait MyNamespace<__Table__> {
            type Delegate;
        }
        impl<__Table__> MyNamespace<__Table__> for FooProviderComponent {
            type Delegate = RedirectLookup<
                __Table__,
                PathCons<MyApp, PathCons<MyFooComponent, Nil>>,
            >;
        }
        ")
    }
}

snapshot_cgp_namespace! {
    cgp_namespace! {
        new OtherNamespace {
            FooProviderComponent =>
                @my_app.MyFooComponent,
        }
    }

    expand_multi_ns_other_namespace(output) {
        insta::assert_snapshot!(output, @"
        pub struct __OtherNamespaceComponents;
        pub trait OtherNamespace<__Table__> {
            type Delegate;
        }
        impl<__Table__> OtherNamespace<__Table__> for FooProviderComponent {
            type Delegate = RedirectLookup<
                __Table__,
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            'm',
                            Chars<'y', Chars<'_', Chars<'a', Chars<'p', Chars<'p', Nil>>>>>,
                        >,
                    >,
                    PathCons<MyFooComponent, Nil>,
                >,
            >;
        }
        ")
    }
}

snapshot_cgp_component! {
    #[cgp_component(BarProvider)]
    #[prefix(@MyApp.MyBarComponent in MyNamespace)]
    #[prefix(@my_app.MyBarComponent in OtherNamespace)]
    pub trait Bar {
        fn bar(&self);
    }

    expand_multi_ns_bar(output) {
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
                PathCons<MyApp, PathCons<MyBarComponent, PathCons<BarProviderComponent, Nil>>>,
            >;
        }
        impl<__Components__> OtherNamespace<__Components__> for BarProviderComponent {
            type Delegate = RedirectLookup<
                __Components__,
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            'm',
                            Chars<'y', Chars<'_', Chars<'a', Chars<'p', Chars<'p', Nil>>>>>,
                        >,
                    >,
                    PathCons<MyBarComponent, PathCons<BarProviderComponent, Nil>>,
                >,
            >;
        }
        ")
    }
}

pub struct MyFooComponent;

pub struct MyBarComponent;

// Incidental: plain providers wired to the namespaces below.
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

            @MyApp.MyFooComponent:
                DummyFoo,
            @MyApp.MyBarComponent:
                DummyBar,
        }
    }

    expand_multi_ns_app(output) {
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
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<MyApp, PathCons<MyFooComponent, __Wildcard__>>> for App {
            type Delegate = DummyFoo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<MyApp, PathCons<MyFooComponent, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyFoo: IsProviderFor<
                PathCons<MyApp, PathCons<MyFooComponent, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<MyApp, PathCons<MyBarComponent, __Wildcard__>>> for App {
            type Delegate = DummyBar;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<MyApp, PathCons<MyBarComponent, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyBar: IsProviderFor<
                PathCons<MyApp, PathCons<MyBarComponent, __Wildcard__>>,
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

pub struct OtherApp;

snapshot_delegate_components! {
    delegate_components! {
        OtherApp {
            namespace OtherNamespace;

            @my_app.MyFooComponent:
                DummyFoo,
            @my_app.MyBarComponent:
                DummyBar,
        }
    }

    expand_multi_ns_other_app(output) {
        insta::assert_snapshot!(output, @"
        impl<__Key__, __Value__> DelegateComponent<__Key__> for OtherApp
        where
            __Key__: OtherNamespace<OtherApp, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        impl<
            __Key__,
            __Value__,
            __Context__,
            __Params__,
        > IsProviderFor<__Key__, __Context__, __Params__> for OtherApp
        where
            __Key__: OtherNamespace<OtherApp, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<
                    6,
                    Chars<'m', Chars<'y', Chars<'_', Chars<'a', Chars<'p', Chars<'p', Nil>>>>>>,
                >,
                PathCons<MyFooComponent, __Wildcard__>,
            >,
        > for OtherApp {
            type Delegate = DummyFoo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<
                    6,
                    Chars<'m', Chars<'y', Chars<'_', Chars<'a', Chars<'p', Chars<'p', Nil>>>>>>,
                >,
                PathCons<MyFooComponent, __Wildcard__>,
            >,
            __Context__,
            __Params__,
        > for OtherApp
        where
            DummyFoo: IsProviderFor<
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            'm',
                            Chars<'y', Chars<'_', Chars<'a', Chars<'p', Chars<'p', Nil>>>>>,
                        >,
                    >,
                    PathCons<MyFooComponent, __Wildcard__>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<
                    6,
                    Chars<'m', Chars<'y', Chars<'_', Chars<'a', Chars<'p', Chars<'p', Nil>>>>>>,
                >,
                PathCons<MyBarComponent, __Wildcard__>,
            >,
        > for OtherApp {
            type Delegate = DummyBar;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<
                    6,
                    Chars<'m', Chars<'y', Chars<'_', Chars<'a', Chars<'p', Chars<'p', Nil>>>>>>,
                >,
                PathCons<MyBarComponent, __Wildcard__>,
            >,
            __Context__,
            __Params__,
        > for OtherApp
        where
            DummyBar: IsProviderFor<
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            'm',
                            Chars<'y', Chars<'_', Chars<'a', Chars<'p', Chars<'p', Nil>>>>>,
                        >,
                    >,
                    PathCons<MyBarComponent, __Wildcard__>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

check_components! {
    OtherApp {
        FooProviderComponent,
        BarProviderComponent,
    }
}
