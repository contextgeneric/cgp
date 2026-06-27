use cgp::prelude::{DefaultNamespace, cgp_component, cgp_impl, check_components};
use cgp_macro_test_util::snapshot_delegate_components;

#[cgp_component(FooProvider)]
#[prefix(@app in DefaultNamespace)]
pub trait Foo<T> {
    fn foo(&self, value: &T);
}

#[cgp_component(BarProvider)]
#[prefix(@app in DefaultNamespace)]
pub trait Bar<T> {
    fn bar(&self, value: &T);
}

pub struct DummyImpl;

#[cgp_impl(DummyImpl)]
impl<T> FooProvider<T> {
    fn foo(&self, _value: &T) {}
}

#[cgp_impl(DummyImpl)]
impl<T> BarProvider<T> {
    fn bar(&self, _value: &T) {}
}

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            namespace DefaultNamespace;

            @app.[FooProviderComponent, BarProviderComponent].[u64, String]:
                DummyImpl,
        }
    }

    expand_delegate_components(output) {
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
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<u64, __Wildcard__>>,
            >,
        > for App {
            type Delegate = DummyImpl;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<u64, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            DummyImpl: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<FooProviderComponent, PathCons<u64, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>,
            >,
        > for App {
            type Delegate = DummyImpl;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            DummyImpl: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<BarProviderComponent, PathCons<u64, __Wildcard__>>,
            >,
        > for App {
            type Delegate = DummyImpl;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<BarProviderComponent, PathCons<u64, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            DummyImpl: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<BarProviderComponent, PathCons<u64, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<BarProviderComponent, PathCons<String, __Wildcard__>>,
            >,
        > for App {
            type Delegate = DummyImpl;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<BarProviderComponent, PathCons<String, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            DummyImpl: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<BarProviderComponent, PathCons<String, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

check_components! {
    App {
        [FooProviderComponent, BarProviderComponent]:
            [u64, String]
    }
}
