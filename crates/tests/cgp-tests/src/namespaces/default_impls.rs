use core::fmt::Display;

use cgp::core::component::DefaultImpls1;
use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_component, snapshot_cgp_impl, snapshot_cgp_namespace};

snapshot_cgp_component! {
    #[cgp_component(ShowImpl)]
    #[prefix(@test in DefaultNamespace)]
    pub trait Show<T> {
        fn show(&self, value: &T) -> String;
    }

    expand_show(output) {
        insta::assert_snapshot!(output, @"
        pub trait Show<T> {
            fn show(&self, value: &T) -> String;
        }
        impl<__Context__, T> Show<T> for __Context__
        where
            __Context__: ShowImpl<__Context__, T>,
        {
            fn show(&self, value: &T) -> String {
                __Context__::show(self, value)
            }
        }
        pub trait ShowImpl<__Context__, T>: IsProviderFor<ShowImplComponent, __Context__, (T)> {
            fn show(__context__: &__Context__, value: &T) -> String;
        }
        impl<__Provider__, __Context__, T> ShowImpl<__Context__, T> for __Provider__
        where
            __Provider__: DelegateComponent<ShowImplComponent>
                + IsProviderFor<ShowImplComponent, __Context__, (T)>,
            <__Provider__ as DelegateComponent<
                ShowImplComponent,
            >>::Delegate: ShowImpl<__Context__, T>,
        {
            fn show(__context__: &__Context__, value: &T) -> String {
                <__Provider__ as DelegateComponent<
                    ShowImplComponent,
                >>::Delegate::show(__context__, value)
            }
        }
        pub struct ShowImplComponent;
        impl<__Context__, T> ShowImpl<__Context__, T> for UseContext
        where
            __Context__: Show<T>,
        {
            fn show(__context__: &__Context__, value: &T) -> String {
                __Context__::show(__context__, value)
            }
        }
        impl<__Context__, T> IsProviderFor<ShowImplComponent, __Context__, (T)> for UseContext
        where
            __Context__: Show<T>,
        {}
        impl<__Context__, T, __Components__, __Path__> ShowImpl<__Context__, T>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<T, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >>::Delegate: ShowImpl<__Context__, T>,
        {
            fn show(__context__: &__Context__, value: &T) -> String {
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
                >>::Delegate::show(__context__, value)
            }
        }
        impl<
            __Context__,
            T,
            __Components__,
            __Path__,
        > IsProviderFor<ShowImplComponent, __Context__, (T)>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<T, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >>::Delegate: IsProviderFor<ShowImplComponent, __Context__, (T)>
                + ShowImpl<__Context__, T>,
        {}
        impl<__Components__> DefaultNamespace<__Components__> for ShowImplComponent {
            type Delegate = RedirectLookup<
                __Components__,
                PathCons<
                    Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                    PathCons<ShowImplComponent, Nil>,
                >,
            >;
        }
        ")
    }
}

snapshot_cgp_impl! {
    #[cgp_impl(new ShowString)]
    #[default_impl(String in DefaultImpls1<ShowImplComponent>)]
    impl ShowImpl<String> {
        fn show(&self, value: &String) -> String {
            value.clone()
        }
    }

    expand_show_string(output) {
        insta::assert_snapshot!(output, @"
        impl<__Context__> ShowImpl<__Context__, String> for ShowString {
            fn show(__context__: &__Context__, value: &String) -> String {
                value.clone()
            }
        }
        impl<__Context__> IsProviderFor<ShowImplComponent, __Context__, (String)>
        for ShowString {}
        pub struct ShowString;
        impl<__Components__> DefaultImpls1<ShowImplComponent, __Components__> for String {
            type Delegate = ShowString;
        }
        ")
    }
}

snapshot_cgp_impl! {
    #[cgp_impl(new ShowWithDisplay)]
    impl<T: Display> ShowImpl<T> {
        fn show(&self, value: &T) -> String {
            value.to_string()
        }
    }

    expand_show_with_display(output) {
        insta::assert_snapshot!(output, @"
        impl<__Context__, T: Display> ShowImpl<__Context__, T> for ShowWithDisplay {
            fn show(__context__: &__Context__, value: &T) -> String {
                value.to_string()
            }
        }
        impl<__Context__, T: Display> IsProviderFor<ShowImplComponent, __Context__, (T)>
        for ShowWithDisplay {}
        pub struct ShowWithDisplay;
        ")
    }
}

snapshot_cgp_namespace! {
    cgp_namespace! {
        new DefaultShowComponents {
            [
                String,
                u64,
            ]:
                ShowWithDisplay,
        }
    }

    expand_default_show_components(output) {
        insta::assert_snapshot!(output, @"
        pub struct __DefaultShowComponentsComponents;
        pub trait DefaultShowComponents<__Table__> {
            type Delegate;
        }
        impl<__Table__> DefaultShowComponents<__Table__> for String {
            type Delegate = ShowWithDisplay;
        }
        impl<__Table__> DefaultShowComponents<__Table__> for u64 {
            type Delegate = ShowWithDisplay;
        }
        ")
    }
}

snapshot_cgp_namespace! {
    cgp_namespace! {
        new ExtendedNamespace: DefaultNamespace {
        }
    }

    expand_default_impls_extended_namespace(output) {
        insta::assert_snapshot!(output, @"
        pub struct __ExtendedNamespaceComponents;
        pub trait ExtendedNamespace<__Table__> {
            type Delegate;
        }
        impl<__Table__, __Key__, __Value__> ExtendedNamespace<__Table__> for __Key__
        where
            __Key__: DefaultNamespace<__ExtendedNamespaceComponents>,
            __Key__: DefaultNamespace<__Table__, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        ")
    }
}

snapshot_cgp_impl! {
    #[cgp_impl(new ShowU32)]
    #[default_impl(@test.ShowImplComponent.u32 in ExtendedNamespace)]
    impl ShowImpl<u32> {
        fn show(&self, value: &u32) -> String {
            value.to_string()
        }
    }

    expand_show_u32(output) {
        insta::assert_snapshot!(output, @"
        impl<__Context__> ShowImpl<__Context__, u32> for ShowU32 {
            fn show(__context__: &__Context__, value: &u32) -> String {
                value.to_string()
            }
        }
        impl<__Context__> IsProviderFor<ShowImplComponent, __Context__, (u32)> for ShowU32 {}
        pub struct ShowU32;
        impl<__Components__> ExtendedNamespace<__Components__>
        for PathCons<
            Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
            PathCons<ShowImplComponent, PathCons<u32, Nil>>,
        > {
            type Delegate = ShowU32;
        }
        ")
    }
}
