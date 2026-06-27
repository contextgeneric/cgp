use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_check_components, snapshot_delegate_components};

#[cgp_component(FooProvider)]
#[prefix(@app in DefaultNamespace)]
pub trait Foo<'a, T, U> {
    fn foo(&self, first: &'a T, second: U);
}

#[cgp_impl(new DummyFoo)]
impl<'a, T, U> FooProvider<'a, T, U> {
    fn foo(&self, _first: &'a T, _second: U) {}
}

pub struct AppA;

snapshot_delegate_components! {
    delegate_components! {
        AppA {
            open {FooProviderComponent};

            @FooProviderComponent.String.u32:
                DummyFoo,
            <T> @FooProviderComponent.bool.T:
                DummyFoo,
        }
    }

    expand_multi_param_app_a(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<FooProviderComponent> for AppA {
            type Delegate = RedirectLookup<AppA, PathCons<FooProviderComponent, Nil>>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<FooProviderComponent, __Context__, __Params__> for AppA
        where
            RedirectLookup<
                AppA,
                PathCons<FooProviderComponent, Nil>,
            >: IsProviderFor<FooProviderComponent, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<FooProviderComponent, PathCons<String, PathCons<u32, __Wildcard__>>>,
        > for AppA {
            type Delegate = DummyFoo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<FooProviderComponent, PathCons<String, PathCons<u32, __Wildcard__>>>,
            __Context__,
            __Params__,
        > for AppA
        where
            DummyFoo: IsProviderFor<
                PathCons<FooProviderComponent, PathCons<String, PathCons<u32, __Wildcard__>>>,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            T,
            __Wildcard__,
        > DelegateComponent<
            PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
        > for AppA {
            type Delegate = DummyFoo;
        }
        impl<
            T,
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
            __Context__,
            __Params__,
        > for AppA
        where
            DummyFoo: IsProviderFor<
                PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

snapshot_check_components! {
    check_components! {
        AppA {
            FooProviderComponent: [
                <'a> (Life<'a>, String, u32),
                <'a> (Life<'a>, bool, String),
            ],
            FooProviderComponent:
                <'a> (Life<'a>, bool, bool),
        }
    }

    expand_check_app_a(output) {
        insta::assert_snapshot!(output, @"
        trait __CheckAppA<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl<'a> __CheckAppA<FooProviderComponent, (Life<'a>, String, u32)> for AppA {}
        impl<'a> __CheckAppA<FooProviderComponent, (Life<'a>, bool, String)> for AppA {}
        impl<'a> __CheckAppA<FooProviderComponent, (Life<'a>, bool, bool)> for AppA {}
        ")
    }
}

pub struct AppB;

snapshot_delegate_components! {
    delegate_components! {
        AppB {
            namespace DefaultNamespace;

            @app.FooProviderComponent.String.u64:
                DummyFoo,
            @app.FooProviderComponent.bool.<T> T:
                DummyFoo,
        }
    }

    expand_multi_param_app_b(output) {
        insta::assert_snapshot!(output, @"
        impl<__Key__, __Value__> DelegateComponent<__Key__> for AppB
        where
            __Key__: DefaultNamespace<AppB, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        impl<
            __Key__,
            __Value__,
            __Context__,
            __Params__,
        > IsProviderFor<__Key__, __Context__, __Params__> for AppB
        where
            __Key__: DefaultNamespace<AppB, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<String, PathCons<u64, __Wildcard__>>>,
            >,
        > for AppB {
            type Delegate = DummyFoo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<String, PathCons<u64, __Wildcard__>>>,
            >,
            __Context__,
            __Params__,
        > for AppB
        where
            DummyFoo: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<FooProviderComponent, PathCons<String, PathCons<u64, __Wildcard__>>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            T,
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
            >,
        > for AppB {
            type Delegate = DummyFoo;
        }
        impl<
            T,
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
            >,
            __Context__,
            __Params__,
        > for AppB
        where
            DummyFoo: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

snapshot_check_components! {
    check_components! {
        <'a> AppB {
            FooProviderComponent: [
                (Life<'a>, String, u64),
                (Life<'a>, bool, String),
            ],
        }
    }

    expand_check_app_b(output) {
        insta::assert_snapshot!(output, @"
        trait __CheckAppB<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl<'a> __CheckAppB<FooProviderComponent, (Life<'a>, String, u64)> for AppB {}
        impl<'a> __CheckAppB<FooProviderComponent, (Life<'a>, bool, String)> for AppB {}
        ")
    }
}
