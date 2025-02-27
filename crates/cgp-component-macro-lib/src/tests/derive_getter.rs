use quote::quote;

use crate::cgp_getter;
use crate::tests::helper::equal::assert_equal_token_stream;

#[test]
fn test_derive_getter_basic() {
    let derived = cgp_getter(
        quote! {
            provider: NameGetter,
        },
        quote! {
            pub trait HasName: HasNameType {
                fn name(&self) -> &Self::Name;
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub struct NameGetterComponent;

        pub trait HasName: HasNameType {
            fn name(&self) -> &Self::Name;
        }

        pub trait NameGetter<Context>: IsProviderFor<NameGetterComponent, Context, ()>
        where
            Context: HasNameType,
        {
            fn name(context: &Context) -> &Context::Name;
        }

        impl<Context> HasName for Context
        where
            Context: HasNameType,
            Context: HasProvider,
            Context::Provider: NameGetter<Context>,
        {
            fn name(&self) -> &Self::Name {
                Context::Provider::name(self)
            }
        }

        impl<Component, Context> NameGetter<Context> for Component
        where
            Context: HasNameType,
            Component: DelegateComponent<NameGetterComponent>
                + IsProviderFor<NameGetterComponent, Context, ()>,
            Component::Delegate: NameGetter<Context>,
        {
            fn name(context: &Context) -> &Context::Name {
                Component::Delegate::name(context)
            }
        }

        impl<Context> NameGetter<Context> for UseFields
        where
            Context: HasNameType,
            Context: HasField<
                Cons<Char<'n'>, Cons<Char<'a'>, Cons<Char<'m'>, Cons<Char<'e'>, Nil>>>>,
                Value = Context::Name,
            >,
        {
            fn name(context: &Context) -> &Context::Name {
                context.get_field(
                    ::core::marker::PhantomData::<
                        Cons<Char<'n'>, Cons<Char<'a'>, Cons<Char<'m'>, Cons<Char<'e'>, Nil>>>>,
                    >,
                )
            }
        }

        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseFields
        where
            Context: HasNameType,
            Context: HasField<
                Cons<Char<'n'>, Cons<Char<'a'>, Cons<Char<'m'>, Cons<Char<'e'>, Nil>>>>,
                Value = Context::Name,
            >,
        {
        }

        impl<Context, __Tag__> NameGetter<Context> for UseField<__Tag__>
        where
            Context: HasNameType,
            Context: HasField<__Tag__, Value = Context::Name>,
        {
            fn name(context: &Context) -> &Context::Name {
                context.get_field(::core::marker::PhantomData)
            }
        }

        impl<Context, __Tag__> IsProviderFor<NameGetterComponent, Context, ()> for UseField<__Tag__>
        where
            Context: HasNameType,
            Context: HasField<__Tag__, Value = Context::Name>,
        {
        }

        impl<Context, __Provider__> NameGetter<Context> for WithProvider<__Provider__>
        where
            Context: HasNameType,
            __Provider__: FieldGetter<Context, NameGetterComponent, Value = Context::Name>,
        {
            fn name(context: &Context) -> &Context::Name {
                __Provider__::get_field(context, ::core::marker::PhantomData)
            }
        }

        impl<Context, __Provider__> IsProviderFor<NameGetterComponent, Context, ()>
            for WithProvider<__Provider__>
        where
            Context: HasNameType,
            __Provider__: FieldGetter<Context, NameGetterComponent, Value = Context::Name>,
        {
        }
    };

    assert_equal_token_stream(&derived, &expected);
}

#[test]
fn test_derive_getter_with_generics() {
    let derived = cgp_getter(
        quote! {
            provider: NameGetter,
        },
        quote! {
            pub trait HasName<App>
            where
                App: HasNameType,
            {
                fn name(&self) -> &App::Name;
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub struct NameGetterComponent;

        pub trait HasName<App>
        where
            App: HasNameType,
        {
            fn name(&self) -> &App::Name;
        }

        pub trait NameGetter<Context, App>: IsProviderFor<NameGetterComponent, Context, (App)>
        where
            App: HasNameType,
        {
            fn name(context: &Context) -> &App::Name;
        }

        impl<Context, App> HasName<App> for Context
        where
            App: HasNameType,
            Context: HasProvider,
            Context::Provider: NameGetter<Context, App>,
        {
            fn name(&self) -> &App::Name {
                Context::Provider::name(self)
            }
        }

        impl<Component, Context, App> NameGetter<Context, App> for Component
        where
            App: HasNameType,
            Component: DelegateComponent<NameGetterComponent>
                + IsProviderFor<NameGetterComponent, Context, (App)>,
            Component::Delegate: NameGetter<Context, App>,
        {
            fn name(context: &Context) -> &App::Name {
                Component::Delegate::name(context)
            }
        }

        impl<Context, App> NameGetter<Context, App> for UseFields
        where
            App: HasNameType,
            Context: HasField<
                Cons<Char<'n'>, Cons<Char<'a'>, Cons<Char<'m'>, Cons<Char<'e'>, Nil>>>>,
                Value = App::Name,
            >,
        {
            fn name(context: &Context) -> &App::Name {
                context.get_field(
                    ::core::marker::PhantomData::<
                        Cons<Char<'n'>, Cons<Char<'a'>, Cons<Char<'m'>, Cons<Char<'e'>, Nil>>>>,
                    >,
                )
            }
        }

        impl<Context, App> IsProviderFor<NameGetterComponent, Context, (App)> for UseFields
        where
            App: HasNameType,
            Context: HasField<
                Cons<Char<'n'>, Cons<Char<'a'>, Cons<Char<'m'>, Cons<Char<'e'>, Nil>>>>,
                Value = App::Name,
            >,
        {
        }

        impl<Context, App, __Tag__> NameGetter<Context, App> for UseField<__Tag__>
        where
            App: HasNameType,
            Context: HasField<__Tag__, Value = App::Name>,
        {
            fn name(context: &Context) -> &App::Name {
                context.get_field(::core::marker::PhantomData)
            }
        }

        impl<Context, App, __Tag__> IsProviderFor<NameGetterComponent, Context, (App)> for UseField<__Tag__>
        where
            App: HasNameType,
            Context: HasField<__Tag__, Value = App::Name>,
        {
        }

        impl<Context, App, __Provider__> NameGetter<Context, App> for WithProvider<__Provider__>
        where
            App: HasNameType,
            __Provider__: FieldGetter<Context, NameGetterComponent, Value = App::Name>,
        {
            fn name(context: &Context) -> &App::Name {
                __Provider__::get_field(context, ::core::marker::PhantomData)
            }
        }

        impl<Context, App, __Provider__> IsProviderFor<NameGetterComponent, Context, (App)>
            for WithProvider<__Provider__>
        where
            App: HasNameType,
            __Provider__: FieldGetter<Context, NameGetterComponent, Value = App::Name>,
        {
        }
    };

    assert_equal_token_stream(&derived, &expected);
}
