use cgp_macro_test_util::{snapshot_cgp_auto_getter, snapshot_cgp_component, snapshot_cgp_impl};

snapshot_cgp_component! {
    #[cgp_component(FooProvider)]
    pub trait CanDoFoo {
        fn foo(&self, value: u32) -> String;
    }

    expand_foo_component(output) {
        insta::assert_snapshot!(output, @"
        pub trait CanDoFoo {
            fn foo(&self, value: u32) -> String;
        }
        impl<__Context__> CanDoFoo for __Context__
        where
            __Context__: FooProvider<__Context__>,
        {
            fn foo(&self, value: u32) -> String {
                __Context__::foo(self, value)
            }
        }
        pub trait FooProvider<
            __Context__,
        >: IsProviderFor<FooProviderComponent, __Context__, ()> {
            fn foo(__context__: &__Context__, value: u32) -> String;
        }
        impl<__Provider__, __Context__> FooProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<FooProviderComponent>
                + IsProviderFor<FooProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                FooProviderComponent,
            >>::Delegate: FooProvider<__Context__>,
        {
            fn foo(__context__: &__Context__, value: u32) -> String {
                <__Provider__ as DelegateComponent<
                    FooProviderComponent,
                >>::Delegate::foo(__context__, value)
            }
        }
        pub struct FooProviderComponent;
        impl<__Context__> FooProvider<__Context__> for UseContext
        where
            __Context__: CanDoFoo,
        {
            fn foo(__context__: &__Context__, value: u32) -> String {
                __Context__::foo(__context__, value)
            }
        }
        impl<__Context__> IsProviderFor<FooProviderComponent, __Context__, ()> for UseContext
        where
            __Context__: CanDoFoo,
        {}
        impl<__Context__, __Components__, __Path__> FooProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: FooProvider<__Context__>,
        {
            fn foo(__context__: &__Context__, value: u32) -> String {
                <__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate::foo(__context__, value)
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

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasName {
        fn name(&self) -> &str;
    }

    expand_has_name(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasName {
            fn name(&self) -> &str;
        }
        impl<__Context__> HasName for __Context__
        where
            __Context__: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = String,
            >,
        {
            fn name(&self) -> &str {
                self.get_field(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    )
                    .as_str()
            }
        }
        ")
    }
}

snapshot_cgp_impl! {
    #[cgp_impl(new ValueToString)]
    impl<Context> FooProvider for Context {
        fn foo(&self, value: u32) -> String {
            value.to_string()
        }
    }

    expand_value_to_string(output) {
        insta::assert_snapshot!(output, @"
        impl<Context> FooProvider<Context> for ValueToString {
            fn foo(__context__: &Context, value: u32) -> String {
                value.to_string()
            }
        }
        impl<Context> IsProviderFor<FooProviderComponent, Context, ()> for ValueToString {}
        pub struct ValueToString;
        ")
    }
}

pub mod inner {
    use core::fmt::Display;

    use cgp::prelude::*;

    use super::{FooProvider, FooProviderComponent, HasName};

    #[cgp_impl(new WithNamePrefix)]
    impl<Context> FooProvider for Context
    where
        Context: HasName,
    {
        fn foo(&self, value: u32) -> String {
            format!("{}: {}", self.name(), value)
        }
    }

    pub struct Foo<Tag> {
        pub tag: Tag,
    }

    #[cgp_impl(new WithFooTag: FooProviderComponent)]
    impl<Tag> FooProvider for Foo<Tag>
    where
        Tag: Display,
    {
        fn foo(&self, value: u32) -> String {
            format!("{}: {}", self.tag, value)
        }
    }
}
