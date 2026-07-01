//! `#[cgp_getter { provider: GetString, name: GetStringComp }]` overrides both
//! the provider name and the component marker name independently, via the brace
//! spec form. The consumer trait and its `&str` return shorthand are unchanged.
//!
//! See docs/reference/macros/cgp_getter.md and docs/reference/providers/use_field.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_getter;

snapshot_cgp_getter! {
    #[cgp_getter{
        provider: GetString,
        name: GetStringComp,
    }]
    pub trait HasFoo {
        fn foo(&self) -> &str;
    }

    expand_has_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFoo {
            fn foo(&self) -> &str;
        }
        impl<__Context__> HasFoo for __Context__
        where
            __Context__: GetString<__Context__>,
        {
            fn foo(&self) -> &str {
                __Context__::foo(self)
            }
        }
        pub trait GetString<__Context__>: IsProviderFor<GetStringComp, __Context__, ()> {
            fn foo(__context__: &__Context__) -> &str;
        }
        impl<__Provider__, __Context__> GetString<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<GetStringComp>
                + IsProviderFor<GetStringComp, __Context__, ()>,
            <__Provider__ as DelegateComponent<GetStringComp>>::Delegate: GetString<__Context__>,
        {
            fn foo(__context__: &__Context__) -> &str {
                <__Provider__ as DelegateComponent<GetStringComp>>::Delegate::foo(__context__)
            }
        }
        pub struct GetStringComp;
        impl<__Context__> GetString<__Context__> for UseContext
        where
            __Context__: HasFoo,
        {
            fn foo(__context__: &__Context__) -> &str {
                __Context__::foo(__context__)
            }
        }
        impl<__Context__> IsProviderFor<GetStringComp, __Context__, ()> for UseContext
        where
            __Context__: HasFoo,
        {}
        impl<__Context__, __Components__, __Path__> GetString<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: GetString<__Context__>,
        {
            fn foo(__context__: &__Context__) -> &str {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::foo(__context__)
            }
        }
        impl<__Context__, __Components__, __Path__> IsProviderFor<GetStringComp, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<GetStringComp, __Context__, ()> + GetString<__Context__>,
        {}
        impl<__Context__> GetString<__Context__> for UseFields
        where
            __Context__: HasField<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value = String,
            >,
        {
            fn foo(__context__: &__Context__) -> &str {
                __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                        >,
                    )
                    .as_str()
            }
        }
        impl<__Context__> IsProviderFor<GetStringComp, __Context__, ()> for UseFields
        where
            __Context__: HasField<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value = String,
            >,
        {}
        impl<__Context__, __Tag__> GetString<__Context__> for UseField<__Tag__>
        where
            __Context__: HasField<__Tag__, Value = String>,
        {
            fn foo(__context__: &__Context__) -> &str {
                __context__.get_field(::core::marker::PhantomData::<__Tag__>).as_str()
            }
        }
        impl<__Context__, __Tag__> IsProviderFor<GetStringComp, __Context__, ()>
        for UseField<__Tag__>
        where
            __Context__: HasField<__Tag__, Value = String>,
        {}
        impl<__Context__, __Provider__> GetString<__Context__> for WithProvider<__Provider__>
        where
            __Provider__: FieldGetter<__Context__, GetStringComp, Value = String>,
        {
            fn foo(__context__: &__Context__) -> &str {
                __Provider__::get_field(
                        __context__,
                        ::core::marker::PhantomData::<GetStringComp>,
                    )
                    .as_str()
            }
        }
        impl<__Context__, __Provider__> IsProviderFor<GetStringComp, __Context__, ()>
        for WithProvider<__Provider__>
        where
            __Provider__: FieldGetter<__Context__, GetStringComp, Value = String>,
        {}
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub bar: String,
}

delegate_components! {
    App {
        GetStringComp: UseField<Symbol!("bar")>,
    }
}

#[test]
pub fn test_string_getter_with_custom_spec() {
    let context = App {
        bar: "abc".to_owned(),
    };

    assert_eq!(context.foo(), "abc");
}
