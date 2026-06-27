use cgp_macro_test_util::{
    snapshot_cgp_component, snapshot_cgp_impl, snapshot_check_components,
    snapshot_delegate_components,
};

pub struct App;

snapshot_cgp_component! {
    #[cgp_component(FooProvider)]
    pub trait Foo<T> {
        fn foo(&self, value: &T);
    }

    expand_open_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait Foo<T> {
            fn foo(&self, value: &T);
        }
        impl<__Context__, T> Foo<T> for __Context__
        where
            __Context__: FooProvider<__Context__, T>,
        {
            fn foo(&self, value: &T) {
                __Context__::foo(self, value)
            }
        }
        pub trait FooProvider<
            __Context__,
            T,
        >: IsProviderFor<FooProviderComponent, __Context__, (T)> {
            fn foo(__context__: &__Context__, value: &T);
        }
        impl<__Provider__, __Context__, T> FooProvider<__Context__, T> for __Provider__
        where
            __Provider__: DelegateComponent<FooProviderComponent>
                + IsProviderFor<FooProviderComponent, __Context__, (T)>,
            <__Provider__ as DelegateComponent<
                FooProviderComponent,
            >>::Delegate: FooProvider<__Context__, T>,
        {
            fn foo(__context__: &__Context__, value: &T) {
                <__Provider__ as DelegateComponent<
                    FooProviderComponent,
                >>::Delegate::foo(__context__, value)
            }
        }
        pub struct FooProviderComponent;
        impl<__Context__, T> FooProvider<__Context__, T> for UseContext
        where
            __Context__: Foo<T>,
        {
            fn foo(__context__: &__Context__, value: &T) {
                __Context__::foo(__context__, value)
            }
        }
        impl<__Context__, T> IsProviderFor<FooProviderComponent, __Context__, (T)> for UseContext
        where
            __Context__: Foo<T>,
        {}
        impl<__Context__, T, __Components__, __Path__> FooProvider<__Context__, T>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<T, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >>::Delegate: FooProvider<__Context__, T>,
        {
            fn foo(__context__: &__Context__, value: &T) {
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
                >>::Delegate::foo(__context__, value)
            }
        }
        impl<
            __Context__,
            T,
            __Components__,
            __Path__,
        > IsProviderFor<FooProviderComponent, __Context__, (T)>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<T, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >>::Delegate: IsProviderFor<FooProviderComponent, __Context__, (T)>
                + FooProvider<__Context__, T>,
        {}
        ")
    }
}

snapshot_cgp_component! {
    #[cgp_component(BarProvider)]
    pub trait Bar<T> {
        fn bar(&self, value: &T);
    }

    expand_open_bar(output) {
        insta::assert_snapshot!(output, @"
        pub trait Bar<T> {
            fn bar(&self, value: &T);
        }
        impl<__Context__, T> Bar<T> for __Context__
        where
            __Context__: BarProvider<__Context__, T>,
        {
            fn bar(&self, value: &T) {
                __Context__::bar(self, value)
            }
        }
        pub trait BarProvider<
            __Context__,
            T,
        >: IsProviderFor<BarProviderComponent, __Context__, (T)> {
            fn bar(__context__: &__Context__, value: &T);
        }
        impl<__Provider__, __Context__, T> BarProvider<__Context__, T> for __Provider__
        where
            __Provider__: DelegateComponent<BarProviderComponent>
                + IsProviderFor<BarProviderComponent, __Context__, (T)>,
            <__Provider__ as DelegateComponent<
                BarProviderComponent,
            >>::Delegate: BarProvider<__Context__, T>,
        {
            fn bar(__context__: &__Context__, value: &T) {
                <__Provider__ as DelegateComponent<
                    BarProviderComponent,
                >>::Delegate::bar(__context__, value)
            }
        }
        pub struct BarProviderComponent;
        impl<__Context__, T> BarProvider<__Context__, T> for UseContext
        where
            __Context__: Bar<T>,
        {
            fn bar(__context__: &__Context__, value: &T) {
                __Context__::bar(__context__, value)
            }
        }
        impl<__Context__, T> IsProviderFor<BarProviderComponent, __Context__, (T)> for UseContext
        where
            __Context__: Bar<T>,
        {}
        impl<__Context__, T, __Components__, __Path__> BarProvider<__Context__, T>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<T, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >>::Delegate: BarProvider<__Context__, T>,
        {
            fn bar(__context__: &__Context__, value: &T) {
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
                >>::Delegate::bar(__context__, value)
            }
        }
        impl<
            __Context__,
            T,
            __Components__,
            __Path__,
        > IsProviderFor<BarProviderComponent, __Context__, (T)>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<T, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >>::Delegate: IsProviderFor<BarProviderComponent, __Context__, (T)>
                + BarProvider<__Context__, T>,
        {}
        ")
    }
}

snapshot_cgp_impl! {
    #[cgp_impl(new DummyFoo)]
    impl<T> FooProvider<T> {
        fn foo(&self, _value: &T) {}
    }

    expand_open_dummy_foo(output) {
        insta::assert_snapshot!(output, @"
        impl<__Context__, T> FooProvider<__Context__, T> for DummyFoo {
            fn foo(__context__: &__Context__, _value: &T) {}
        }
        impl<__Context__, T> IsProviderFor<FooProviderComponent, __Context__, (T)> for DummyFoo {}
        pub struct DummyFoo;
        ")
    }
}

snapshot_cgp_impl! {
    #[cgp_impl(new DummyBar)]
    impl<T> BarProvider<T> {
        fn bar(&self, _value: &T) {}
    }

    expand_open_dummy_bar(output) {
        insta::assert_snapshot!(output, @"
        impl<__Context__, T> BarProvider<__Context__, T> for DummyBar {
            fn bar(__context__: &__Context__, _value: &T) {}
        }
        impl<__Context__, T> IsProviderFor<BarProviderComponent, __Context__, (T)> for DummyBar {}
        pub struct DummyBar;
        ")
    }
}

snapshot_delegate_components! {
    delegate_components! {
        App {
            open {FooProviderComponent, BarProviderComponent};

            // FooProviderComponent =>
            //     @FooProviderComponent,
            // BarProviderComponent =>
            //     @BarProviderComponent,

            @FooProviderComponent.String:
                DummyFoo,
            @BarProviderComponent.{u32, u64, bool, usize, isize}:
                DummyBar,
        }
    }

    expand_open_app(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<FooProviderComponent> for App {
            type Delegate = RedirectLookup<App, PathCons<FooProviderComponent, Nil>>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<FooProviderComponent, __Context__, __Params__> for App
        where
            RedirectLookup<
                App,
                PathCons<FooProviderComponent, Nil>,
            >: IsProviderFor<FooProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<BarProviderComponent> for App {
            type Delegate = RedirectLookup<App, PathCons<BarProviderComponent, Nil>>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<BarProviderComponent, __Context__, __Params__> for App
        where
            RedirectLookup<
                App,
                PathCons<BarProviderComponent, Nil>,
            >: IsProviderFor<BarProviderComponent, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>>
        for App {
            type Delegate = DummyFoo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyFoo: IsProviderFor<
                PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<BarProviderComponent, PathCons<u32, __Wildcard__>>>
        for App {
            type Delegate = DummyBar;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<BarProviderComponent, PathCons<u32, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyBar: IsProviderFor<
                PathCons<BarProviderComponent, PathCons<u32, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<BarProviderComponent, PathCons<u64, __Wildcard__>>>
        for App {
            type Delegate = DummyBar;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<BarProviderComponent, PathCons<u64, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyBar: IsProviderFor<
                PathCons<BarProviderComponent, PathCons<u64, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<BarProviderComponent, PathCons<bool, __Wildcard__>>>
        for App {
            type Delegate = DummyBar;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<BarProviderComponent, PathCons<bool, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyBar: IsProviderFor<
                PathCons<BarProviderComponent, PathCons<bool, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<BarProviderComponent, PathCons<usize, __Wildcard__>>>
        for App {
            type Delegate = DummyBar;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<BarProviderComponent, PathCons<usize, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyBar: IsProviderFor<
                PathCons<BarProviderComponent, PathCons<usize, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<BarProviderComponent, PathCons<isize, __Wildcard__>>>
        for App {
            type Delegate = DummyBar;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<BarProviderComponent, PathCons<isize, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyBar: IsProviderFor<
                PathCons<BarProviderComponent, PathCons<isize, __Wildcard__>>,
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
            FooProviderComponent:
                String,
            BarProviderComponent: [
                u32,
                u64,
                bool,
                usize,
                isize,
            ],
        }
    }

    expand_check_app(output) {
        insta::assert_snapshot!(output, @"
        trait __CheckApp<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CheckApp<FooProviderComponent, String> for App {}
        impl __CheckApp<BarProviderComponent, u32> for App {}
        impl __CheckApp<BarProviderComponent, u64> for App {}
        impl __CheckApp<BarProviderComponent, bool> for App {}
        impl __CheckApp<BarProviderComponent, usize> for App {}
        impl __CheckApp<BarProviderComponent, isize> for App {}
        ")
    }
}
