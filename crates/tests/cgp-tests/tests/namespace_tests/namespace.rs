use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::ReturnError;
use cgp::prelude::*;
use cgp_macro_test_util::{
    snapshot_cgp_component, snapshot_check_components, snapshot_delegate_components,
};

pub struct MyComponents;

snapshot_cgp_component! {
    #[cgp_component(FooProvider)]
    #[prefix(@app.MyComponents.FooProviderComponent in DefaultNamespace)]
    pub trait Foo {
        fn foo(&self);
    }

    expand_namespace_foo(output) {
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
        impl<__Components__> DefaultNamespace<__Components__> for FooProviderComponent {
            type Delegate = RedirectLookup<
                __Components__,
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<
                        MyComponents,
                        PathCons<FooProviderComponent, PathCons<FooProviderComponent, Nil>>,
                    >,
                >,
            >;
        }
        ")
    }
}

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            namespace DefaultNamespace;

            @cgp.core.error.ErrorTypeProviderComponent:
                UseType<String>,
            @cgp.core.error.ErrorRaiserComponent.String:
                ReturnError,
        }
    }

    expand_namespace_app(output) {
        insta::assert_snapshot!(output, @"
        impl<__Key__, __Value__> DelegateComponent<__Key__> for App
        where
            __Key__: DefaultNamespace<App, Delegate = __Value__>,
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
            __Key__: DefaultNamespace<App, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'c', Chars<'g', Chars<'p', Nil>>>>,
                PathCons<
                    Symbol<4, Chars<'c', Chars<'o', Chars<'r', Chars<'e', Nil>>>>>,
                    PathCons<
                        Symbol<
                            5,
                            Chars<'e', Chars<'r', Chars<'r', Chars<'o', Chars<'r', Nil>>>>>,
                        >,
                        PathCons<ErrorTypeProviderComponent, __Wildcard__>,
                    >,
                >,
            >,
        > for App {
            type Delegate = UseType<String>;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'c', Chars<'g', Chars<'p', Nil>>>>,
                PathCons<
                    Symbol<4, Chars<'c', Chars<'o', Chars<'r', Chars<'e', Nil>>>>>,
                    PathCons<
                        Symbol<
                            5,
                            Chars<'e', Chars<'r', Chars<'r', Chars<'o', Chars<'r', Nil>>>>>,
                        >,
                        PathCons<ErrorTypeProviderComponent, __Wildcard__>,
                    >,
                >,
            >,
            __Context__,
            __Params__,
        > for App
        where
            UseType<
                String,
            >: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'c', Chars<'g', Chars<'p', Nil>>>>,
                    PathCons<
                        Symbol<4, Chars<'c', Chars<'o', Chars<'r', Chars<'e', Nil>>>>>,
                        PathCons<
                            Symbol<
                                5,
                                Chars<'e', Chars<'r', Chars<'r', Chars<'o', Chars<'r', Nil>>>>>,
                            >,
                            PathCons<ErrorTypeProviderComponent, __Wildcard__>,
                        >,
                    >,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'c', Chars<'g', Chars<'p', Nil>>>>,
                PathCons<
                    Symbol<4, Chars<'c', Chars<'o', Chars<'r', Chars<'e', Nil>>>>>,
                    PathCons<
                        Symbol<
                            5,
                            Chars<'e', Chars<'r', Chars<'r', Chars<'o', Chars<'r', Nil>>>>>,
                        >,
                        PathCons<ErrorRaiserComponent, PathCons<String, __Wildcard__>>,
                    >,
                >,
            >,
        > for App {
            type Delegate = ReturnError;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'c', Chars<'g', Chars<'p', Nil>>>>,
                PathCons<
                    Symbol<4, Chars<'c', Chars<'o', Chars<'r', Chars<'e', Nil>>>>>,
                    PathCons<
                        Symbol<
                            5,
                            Chars<'e', Chars<'r', Chars<'r', Chars<'o', Chars<'r', Nil>>>>>,
                        >,
                        PathCons<ErrorRaiserComponent, PathCons<String, __Wildcard__>>,
                    >,
                >,
            >,
            __Context__,
            __Params__,
        > for App
        where
            ReturnError: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'c', Chars<'g', Chars<'p', Nil>>>>,
                    PathCons<
                        Symbol<4, Chars<'c', Chars<'o', Chars<'r', Chars<'e', Nil>>>>>,
                        PathCons<
                            Symbol<
                                5,
                                Chars<'e', Chars<'r', Chars<'r', Chars<'o', Chars<'r', Nil>>>>>,
                            >,
                            PathCons<ErrorRaiserComponent, PathCons<String, __Wildcard__>>,
                        >,
                    >,
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
        App {
            ErrorRaiserComponent: String,
        }
    }

    expand_check_app(output) {
        insta::assert_snapshot!(output, @"
        trait __CheckApp<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CheckApp<ErrorRaiserComponent, String> for App {}
        ")
    }
}
