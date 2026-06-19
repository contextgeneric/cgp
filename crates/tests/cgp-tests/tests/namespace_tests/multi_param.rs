use cgp::prelude::*;
use cgp_macro_test_util::{
    snapshot_cgp_component, snapshot_cgp_impl, snapshot_check_components,
    snapshot_delegate_components,
};

snapshot_cgp_component! {
    #[cgp_component(FooProvider)]
    #[prefix(@app in DefaultNamespace)]
    pub trait Foo<'a, T, U> {
        fn foo(&self, first: &'a T, second: U);
    }

    expand_multi_param_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait Foo<'a, T, U> {
            fn foo(&self, first: &'a T, second: U);
        }
        impl<'a, __Context__, T, U> Foo<'a, T, U> for __Context__
        where
            __Context__: FooProvider<'a, __Context__, T, U>,
        {
            fn foo(&self, first: &'a T, second: U) {
                __Context__::foo(self, first, second)
            }
        }
        pub trait FooProvider<
            'a,
            __Context__,
            T,
            U,
        >: IsProviderFor<FooProviderComponent, __Context__, (Life<'a>, T, U)> {
            fn foo(__context__: &__Context__, first: &'a T, second: U);
        }
        impl<'a, __Provider__, __Context__, T, U> FooProvider<'a, __Context__, T, U>
        for __Provider__
        where
            __Provider__: DelegateComponent<FooProviderComponent>
                + IsProviderFor<FooProviderComponent, __Context__, (Life<'a>, T, U)>,
            <__Provider__ as DelegateComponent<
                FooProviderComponent,
            >>::Delegate: FooProvider<'a, __Context__, T, U>,
        {
            fn foo(__context__: &__Context__, first: &'a T, second: U) {
                <__Provider__ as DelegateComponent<
                    FooProviderComponent,
                >>::Delegate::foo(__context__, first, second)
            }
        }
        pub struct FooProviderComponent;
        impl<'a, __Context__, T, U> FooProvider<'a, __Context__, T, U> for UseContext
        where
            __Context__: Foo<'a, T, U>,
        {
            fn foo(__context__: &__Context__, first: &'a T, second: U) {
                __Context__::foo(__context__, first, second)
            }
        }
        impl<
            'a,
            __Context__,
            T,
            U,
        > IsProviderFor<FooProviderComponent, __Context__, (Life<'a>, T, U)> for UseContext
        where
            __Context__: Foo<'a, T, U>,
        {}
        impl<'a, __Context__, T, U, __Components__, __Path__> FooProvider<'a, __Context__, T, U>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<T, PathCons<U, Nil>>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, PathCons<U, Nil>>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, PathCons<U, Nil>>>>::Output,
            >>::Delegate: FooProvider<'a, __Context__, T, U>,
        {
            fn foo(__context__: &__Context__, first: &'a T, second: U) {
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<T, PathCons<U, Nil>>>>::Output,
                >>::Delegate::foo(__context__, first, second)
            }
        }
        impl<
            'a,
            __Context__,
            T,
            U,
            __Components__,
            __Path__,
        > IsProviderFor<FooProviderComponent, __Context__, (Life<'a>, T, U)>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<T, PathCons<U, Nil>>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, PathCons<U, Nil>>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, PathCons<U, Nil>>>>::Output,
            >>::Delegate: FooProvider<'a, __Context__, T, U>,
        {}
        impl<__Components__> DefaultNamespace<__Components__> for FooProviderComponent {
            type Delegate = RedirectLookup<
                __Components__,
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<FooProviderComponent, Nil>,
                >,
            >;
        }
        ")
    }
}

snapshot_cgp_impl! {
    #[cgp_impl(new DummyFoo)]
    impl<'a, T, U> FooProvider<'a, T, U> {
        fn foo(&self, _first: &'a T, _second: U) {}
    }

    expand_multi_param_dummy_foo(output) {
        insta::assert_snapshot!(output, @"
        impl<'a, __Context__, T, U> FooProvider<'a, __Context__, T, U> for DummyFoo {
            fn foo(__context__: &__Context__, _first: &'a T, _second: U) {}
        }
        impl<
            'a,
            __Context__,
            T,
            U,
        > IsProviderFor<FooProviderComponent, __Context__, (Life<'a>, T, U)> for DummyFoo {}
        pub struct DummyFoo;
        ")
    }
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
