use cgp_macro_test_util::{
    snapshot_cgp_component, snapshot_cgp_impl, snapshot_cgp_namespace, snapshot_check_components,
    snapshot_delegate_components,
};

snapshot_cgp_component! {
    #[cgp_component(FooProvider)]
    pub trait Foo {
        fn foo(&self);
    }

    expand_basic_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait Foo {
            fn foo(&self);
        }
        impl<__Context__> Foo for __Context__
        where
            __Context__: FooProvider<__Context__>,
        {
            fn foo(&self) {
                __Context__::foo(self)
            }
        }
        pub trait FooProvider<
            __Context__,
        >: IsProviderFor<FooProviderComponent, __Context__, ()> {
            fn foo(__context__: &__Context__);
        }
        impl<__Provider__, __Context__> FooProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<FooProviderComponent>
                + IsProviderFor<FooProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                FooProviderComponent,
            >>::Delegate: FooProvider<__Context__>,
        {
            fn foo(__context__: &__Context__) {
                <__Provider__ as DelegateComponent<
                    FooProviderComponent,
                >>::Delegate::foo(__context__)
            }
        }
        pub struct FooProviderComponent;
        impl<__Context__> FooProvider<__Context__> for UseContext
        where
            __Context__: Foo,
        {
            fn foo(__context__: &__Context__) {
                __Context__::foo(__context__)
            }
        }
        impl<__Context__> IsProviderFor<FooProviderComponent, __Context__, ()> for UseContext
        where
            __Context__: Foo,
        {}
        impl<__Context__, __Components__, __Path__> FooProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: FooProvider<__Context__>,
        {
            fn foo(__context__: &__Context__) {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::foo(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<FooProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<FooProviderComponent, __Context__, ()>
                + FooProvider<__Context__>,
        {}
        ")
    }
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

snapshot_cgp_impl! {
    #[cgp_impl(new DummyFoo)]
    impl FooProvider {
        fn foo(&self) {}
    }

    expand_basic_dummy_foo(output) {
        insta::assert_snapshot!(output, @"
        impl<__Context__> FooProvider<__Context__> for DummyFoo {
            fn foo(__context__: &__Context__) {}
        }
        impl<__Context__> IsProviderFor<FooProviderComponent, __Context__, ()> for DummyFoo {}
        pub struct DummyFoo;
        ")
    }
}

snapshot_cgp_impl! {
    #[cgp_impl(new DummyBar)]
    impl BarProvider {
        fn bar(&self) {}
    }

    expand_basic_dummy_bar(output) {
        insta::assert_snapshot!(output, @"
        impl<__Context__> BarProvider<__Context__> for DummyBar {
            fn bar(__context__: &__Context__) {}
        }
        impl<__Context__> IsProviderFor<BarProviderComponent, __Context__, ()> for DummyBar {}
        pub struct DummyBar;
        ")
    }
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

snapshot_check_components! {
    check_components! {
        App {
            FooProviderComponent,
            BarProviderComponent,
        }
    }

    expand_check_app(output) {
        insta::assert_snapshot!(output, @"
        trait __CheckApp<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CheckApp<FooProviderComponent, ()> for App {}
        impl __CheckApp<BarProviderComponent, ()> for App {}
        ")
    }
}
