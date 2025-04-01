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
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = Context::Name,
            >,
        {
            fn name(context: &Context) -> &Context::Name {
                context.get_field(
                    ::core::marker::PhantomData::<
                        Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                    >,
                )
            }
        }

        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseFields
        where
            Context: HasNameType,
            Context: HasField<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
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
fn test_derive_getter_str() {
    let derived = cgp_getter(
        quote! {
            provider: NameGetter,
        },
        quote! {
            pub trait HasName {
                fn name(&self) -> &str;
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub struct NameGetterComponent;
        pub trait HasName {
            fn name(&self) -> &str;
        }
        pub trait NameGetter<Context>: IsProviderFor<NameGetterComponent, Context, ()> {
            fn name(context: &Context) -> &str;
        }
        impl<Context> HasName for Context
        where
            Context: HasProvider,
            Context::Provider: NameGetter<Context>,
        {
            fn name(&self) -> &str {
                Context::Provider::name(self)
            }
        }
        impl<Component, Context> NameGetter<Context> for Component
        where
            Component: DelegateComponent<NameGetterComponent>
                + IsProviderFor<NameGetterComponent, Context, ()>,
            Component::Delegate: NameGetter<Context>,
        {
            fn name(context: &Context) -> &str {
                Component::Delegate::name(context)
            }
        }
        impl<Context> NameGetter<Context> for UseFields
        where
            Context: HasField<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = String,
            >,
        {
            fn name(context: &Context) -> &str {
                context
                    .get_field(
                        ::core::marker::PhantomData::<
                            Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                        >,
                    )
                    .as_str()
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseFields where
            Context: HasField<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = String,
            >
        {
        }
        impl<Context, __Tag__> NameGetter<Context> for UseField<__Tag__>
        where
            Context: HasField<__Tag__, Value = String>,
        {
            fn name(context: &Context) -> &str {
                context.get_field(::core::marker::PhantomData).as_str()
            }
        }
        impl<Context, __Tag__> IsProviderFor<NameGetterComponent, Context, ()> for UseField<__Tag__> where
            Context: HasField<__Tag__, Value = String>
        {
        }
        impl<Context, __Provider__> NameGetter<Context> for WithProvider<__Provider__>
        where
            __Provider__: FieldGetter<Context, NameGetterComponent, Value = String>,
        {
            fn name(context: &Context) -> &str {
                __Provider__::get_field(context, ::core::marker::PhantomData).as_str()
            }
        }
        impl<Context, __Provider__> IsProviderFor<NameGetterComponent, Context, ()>
            for WithProvider<__Provider__>
        where
            __Provider__: FieldGetter<Context, NameGetterComponent, Value = String>,
        {
        }
    };

    assert_equal_token_stream(&derived, &expected);
}

#[test]
fn test_derive_getter_mut_str() {
    let derived = cgp_getter(
        quote! {
            provider: NameGetter,
        },
        quote! {
            pub trait HasName {
                fn name(&mut self) -> &mut str;
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub struct NameGetterComponent;
        pub trait HasName {
            fn name(&mut self) -> &mut str;
        }
        pub trait NameGetter<Context>: IsProviderFor<NameGetterComponent, Context, ()> {
            fn name(context: &mut Context) -> &mut str;
        }
        impl<Context> HasName for Context
        where
            Context: HasProvider,
            Context::Provider: NameGetter<Context>,
        {
            fn name(&mut self) -> &mut str {
                Context::Provider::name(self)
            }
        }
        impl<Component, Context> NameGetter<Context> for Component
        where
            Component: DelegateComponent<NameGetterComponent>
                + IsProviderFor<NameGetterComponent, Context, ()>,
            Component::Delegate: NameGetter<Context>,
        {
            fn name(context: &mut Context) -> &mut str {
                Component::Delegate::name(context)
            }
        }
        impl<Context> NameGetter<Context> for UseFields
        where
            Context: HasFieldMut<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = String,
            >,
        {
            fn name(context: &mut Context) -> &mut str {
                context
                    .get_field_mut(
                        ::core::marker::PhantomData::<
                            Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                        >,
                    )
                    .as_mut_str()
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseFields where
            Context: HasFieldMut<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = String,
            >
        {
        }
        impl<Context, __Tag__> NameGetter<Context> for UseField<__Tag__>
        where
            Context: HasFieldMut<__Tag__, Value = String>,
        {
            fn name(context: &mut Context) -> &mut str {
                context
                    .get_field_mut(::core::marker::PhantomData)
                    .as_mut_str()
            }
        }
        impl<Context, __Tag__> IsProviderFor<NameGetterComponent, Context, ()> for UseField<__Tag__> where
            Context: HasFieldMut<__Tag__, Value = String>
        {
        }
        impl<Context, __Provider__> NameGetter<Context> for WithProvider<__Provider__>
        where
            __Provider__: MutFieldGetter<Context, NameGetterComponent, Value = String>,
        {
            fn name(context: &mut Context) -> &mut str {
                __Provider__::get_field_mut(context, ::core::marker::PhantomData).as_mut_str()
            }
        }
        impl<Context, __Provider__> IsProviderFor<NameGetterComponent, Context, ()>
            for WithProvider<__Provider__>
        where
            __Provider__: MutFieldGetter<Context, NameGetterComponent, Value = String>,
        {
        }
    };

    assert_equal_token_stream(&derived, &expected);
}

#[test]
fn test_derive_getter_clone() {
    let derived = cgp_getter(
        quote! {
            provider: NameGetter,
        },
        quote! {
            pub trait HasName: HasNameType<Name: Clone> {
                fn name(&self) -> Self::Name;
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub struct NameGetterComponent;
        pub trait HasName: HasNameType<Name: Clone> {
            fn name(&self) -> Self::Name;
        }
        pub trait NameGetter<Context>: IsProviderFor<NameGetterComponent, Context, ()>
        where
            Context: HasNameType<Name: Clone>,
        {
            fn name(context: &Context) -> Context::Name;
        }
        impl<Context> HasName for Context
        where
            Context: HasNameType<Name: Clone>,
            Context: HasProvider,
            Context::Provider: NameGetter<Context>,
        {
            fn name(&self) -> Self::Name {
                Context::Provider::name(self)
            }
        }
        impl<Component, Context> NameGetter<Context> for Component
        where
            Context: HasNameType<Name: Clone>,
            Component: DelegateComponent<NameGetterComponent>
                + IsProviderFor<NameGetterComponent, Context, ()>,
            Component::Delegate: NameGetter<Context>,
        {
            fn name(context: &Context) -> Context::Name {
                Component::Delegate::name(context)
            }
        }
        impl<Context> NameGetter<Context> for UseFields
        where
            Context: HasNameType<Name: Clone>,
            Context: HasField<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = Context::Name,
            >,
        {
            fn name(context: &Context) -> Context::Name {
                context
                    .get_field(
                        ::core::marker::PhantomData::<
                            Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                        >,
                    )
                    .clone()
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseFields
        where
            Context: HasNameType<Name: Clone>,
            Context: HasField<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = Context::Name,
            >,
        {
        }
        impl<Context, __Tag__> NameGetter<Context> for UseField<__Tag__>
        where
            Context: HasNameType<Name: Clone>,
            Context: HasField<__Tag__, Value = Context::Name>,
        {
            fn name(context: &Context) -> Context::Name {
                context.get_field(::core::marker::PhantomData).clone()
            }
        }
        impl<Context, __Tag__> IsProviderFor<NameGetterComponent, Context, ()> for UseField<__Tag__>
        where
            Context: HasNameType<Name: Clone>,
            Context: HasField<__Tag__, Value = Context::Name>,
        {
        }
        impl<Context, __Provider__> NameGetter<Context> for WithProvider<__Provider__>
        where
            Context: HasNameType<Name: Clone>,
            __Provider__: FieldGetter<Context, NameGetterComponent, Value = Context::Name>,
        {
            fn name(context: &Context) -> Context::Name {
                __Provider__::get_field(context, ::core::marker::PhantomData).clone()
            }
        }
        impl<Context, __Provider__> IsProviderFor<NameGetterComponent, Context, ()>
            for WithProvider<__Provider__>
        where
            Context: HasNameType<Name: Clone>,
            __Provider__: FieldGetter<Context, NameGetterComponent, Value = Context::Name>,
        {
        }
    };

    assert_equal_token_stream(&derived, &expected);
}

#[test]
fn test_derive_getter_option_ref() {
    let derived = cgp_getter(
        quote! {
            provider: NameGetter,
        },
        quote! {
            pub trait HasName: HasNameType {
                fn name(&self) -> Option<&Self::Name>;
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub struct NameGetterComponent;
        pub trait HasName: HasNameType {
            fn name(&self) -> Option<&Self::Name>;
        }
        pub trait NameGetter<Context>: IsProviderFor<NameGetterComponent, Context, ()>
        where
            Context: HasNameType,
        {
            fn name(context: &Context) -> Option<&Context::Name>;
        }
        impl<Context> HasName for Context
        where
            Context: HasNameType,
            Context: HasProvider,
            Context::Provider: NameGetter<Context>,
        {
            fn name(&self) -> Option<&Self::Name> {
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
            fn name(context: &Context) -> Option<&Context::Name> {
                Component::Delegate::name(context)
            }
        }
        impl<Context> NameGetter<Context> for UseFields
        where
            Context: HasNameType,
            Context: HasField<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = Option<Context::Name>,
            >,
        {
            fn name(context: &Context) -> Option<&Context::Name> {
                context
                    .get_field(
                        ::core::marker::PhantomData::<
                            Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                        >,
                    )
                    .as_ref()
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseFields
        where
            Context: HasNameType,
            Context: HasField<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = Option<Context::Name>,
            >,
        {
        }
        impl<Context, __Tag__> NameGetter<Context> for UseField<__Tag__>
        where
            Context: HasNameType,
            Context: HasField<__Tag__, Value = Option<Context::Name>>,
        {
            fn name(context: &Context) -> Option<&Context::Name> {
                context.get_field(::core::marker::PhantomData).as_ref()
            }
        }
        impl<Context, __Tag__> IsProviderFor<NameGetterComponent, Context, ()> for UseField<__Tag__>
        where
            Context: HasNameType,
            Context: HasField<__Tag__, Value = Option<Context::Name>>,
        {
        }
        impl<Context, __Provider__> NameGetter<Context> for WithProvider<__Provider__>
        where
            Context: HasNameType,
            __Provider__: FieldGetter<Context, NameGetterComponent, Value = Option<Context::Name>>,
        {
            fn name(context: &Context) -> Option<&Context::Name> {
                __Provider__::get_field(context, ::core::marker::PhantomData).as_ref()
            }
        }
        impl<Context, __Provider__> IsProviderFor<NameGetterComponent, Context, ()>
            for WithProvider<__Provider__>
        where
            Context: HasNameType,
            __Provider__: FieldGetter<Context, NameGetterComponent, Value = Option<Context::Name>>,
        {
        }
    };

    assert_equal_token_stream(&derived, &expected);
}

#[test]
fn test_derive_getter_option_mut() {
    let derived = cgp_getter(
        quote! {
            provider: NameGetter,
        },
        quote! {
            pub trait HasName: HasNameType {
                fn name(&mut self) -> Option<&mut Self::Name>;
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub struct NameGetterComponent;
        pub trait HasName: HasNameType {
            fn name(&mut self) -> Option<&mut Self::Name>;
        }
        pub trait NameGetter<Context>: IsProviderFor<NameGetterComponent, Context, ()>
        where
            Context: HasNameType,
        {
            fn name(context: &mut Context) -> Option<&mut Context::Name>;
        }
        impl<Context> HasName for Context
        where
            Context: HasNameType,
            Context: HasProvider,
            Context::Provider: NameGetter<Context>,
        {
            fn name(&mut self) -> Option<&mut Self::Name> {
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
            fn name(context: &mut Context) -> Option<&mut Context::Name> {
                Component::Delegate::name(context)
            }
        }
        impl<Context> NameGetter<Context> for UseFields
        where
            Context: HasNameType,
            Context: HasFieldMut<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = Option<Context::Name>,
            >,
        {
            fn name(context: &mut Context) -> Option<&mut Context::Name> {
                context
                    .get_field_mut(
                        ::core::marker::PhantomData::<
                            Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                        >,
                    )
                    .as_mut()
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseFields
        where
            Context: HasNameType,
            Context: HasFieldMut<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = Option<Context::Name>,
            >,
        {
        }
        impl<Context, __Tag__> NameGetter<Context> for UseField<__Tag__>
        where
            Context: HasNameType,
            Context: HasFieldMut<__Tag__, Value = Option<Context::Name>>,
        {
            fn name(context: &mut Context) -> Option<&mut Context::Name> {
                context.get_field_mut(::core::marker::PhantomData).as_mut()
            }
        }
        impl<Context, __Tag__> IsProviderFor<NameGetterComponent, Context, ()> for UseField<__Tag__>
        where
            Context: HasNameType,
            Context: HasFieldMut<__Tag__, Value = Option<Context::Name>>,
        {
        }
        impl<Context, __Provider__> NameGetter<Context> for WithProvider<__Provider__>
        where
            Context: HasNameType,
            __Provider__: MutFieldGetter<Context, NameGetterComponent, Value = Option<Context::Name>>,
        {
            fn name(context: &mut Context) -> Option<&mut Context::Name> {
                __Provider__::get_field_mut(context, ::core::marker::PhantomData).as_mut()
            }
        }
        impl<Context, __Provider__> IsProviderFor<NameGetterComponent, Context, ()>
            for WithProvider<__Provider__>
        where
            Context: HasNameType,
            __Provider__: MutFieldGetter<Context, NameGetterComponent, Value = Option<Context::Name>>,
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
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = App::Name,
            >,
        {
            fn name(context: &Context) -> &App::Name {
                context.get_field(
                    ::core::marker::PhantomData::<
                        Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                    >,
                )
            }
        }

        impl<Context, App> IsProviderFor<NameGetterComponent, Context, (App)> for UseFields
        where
            App: HasNameType,
            Context: HasField<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
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

#[test]
fn test_derive_getter_with_component_generics() {
    let derived = cgp_getter(
        quote! {
            name: NameGetterComponent<App>,
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
        pub struct NameGetterComponent<App>(pub core::marker::PhantomData<(App)>);
        pub trait HasName<App>
        where
            App: HasNameType,
        {
            fn name(&self) -> &App::Name;
        }
        pub trait NameGetter<Context, App>:
            IsProviderFor<NameGetterComponent<App>, Context, (App)>
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
            Component: DelegateComponent<NameGetterComponent<App>>
                + IsProviderFor<NameGetterComponent<App>, Context, (App)>,
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
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = App::Name,
            >,
        {
            fn name(context: &Context) -> &App::Name {
                context.get_field(
                    ::core::marker::PhantomData::<
                        Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                    >,
                )
            }
        }
        impl<Context, App> IsProviderFor<NameGetterComponent<App>, Context, (App)> for UseFields
        where
            App: HasNameType,
            Context: HasField<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
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
        impl<Context, App, __Tag__> IsProviderFor<NameGetterComponent<App>, Context, (App)>
            for UseField<__Tag__>
        where
            App: HasNameType,
            Context: HasField<__Tag__, Value = App::Name>,
        {
        }
        impl<Context, App, __Provider__> NameGetter<Context, App> for WithProvider<__Provider__>
        where
            App: HasNameType,
            __Provider__: FieldGetter<Context, NameGetterComponent<App>, Value = App::Name>,
        {
            fn name(context: &Context) -> &App::Name {
                __Provider__::get_field(context, ::core::marker::PhantomData)
            }
        }
        impl<Context, App, __Provider__> IsProviderFor<NameGetterComponent<App>, Context, (App)>
            for WithProvider<__Provider__>
        where
            App: HasNameType,
            __Provider__: FieldGetter<Context, NameGetterComponent<App>, Value = App::Name>,
        {
        }
    };

    assert_equal_token_stream(&derived, &expected);
}

#[test]
fn test_derive_getter_with_phantom() {
    let derived = cgp_getter(
        quote! {
            name: NameGetterComponent,
            provider: NameGetter,
        },
        quote! {
            pub trait HasName<App, B>
            where
                App: HasNameType,
            {
                fn name(&self, _phantom: PhantomData<(App, B)>) -> &App::Name;
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub struct NameGetterComponent;
        pub trait HasName<App, B>
        where
            App: HasNameType,
        {
            fn name(&self, _phantom: PhantomData<(App, B)>) -> &App::Name;
        }
        pub trait NameGetter<Context, App, B>:
            IsProviderFor<NameGetterComponent, Context, (App, B)>
        where
            App: HasNameType,
        {
            fn name(context: &Context, _phantom: PhantomData<(App, B)>) -> &App::Name;
        }
        impl<Context, App, B> HasName<App, B> for Context
        where
            App: HasNameType,
            Context: HasProvider,
            Context::Provider: NameGetter<Context, App, B>,
        {
            fn name(&self, _phantom: PhantomData<(App, B)>) -> &App::Name {
                Context::Provider::name(self, _phantom)
            }
        }
        impl<Component, Context, App, B> NameGetter<Context, App, B> for Component
        where
            App: HasNameType,
            Component: DelegateComponent<NameGetterComponent>
                + IsProviderFor<NameGetterComponent, Context, (App, B)>,
            Component::Delegate: NameGetter<Context, App, B>,
        {
            fn name(context: &Context, _phantom: PhantomData<(App, B)>) -> &App::Name {
                Component::Delegate::name(context, _phantom)
            }
        }
        impl<Context, App, B> NameGetter<Context, App, B> for UseFields
        where
            App: HasNameType,
            Context: HasField<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = App::Name,
            >,
        {
            fn name(context: &Context, _phantom: PhantomData<(App, B)>) -> &App::Name {
                context.get_field(
                    ::core::marker::PhantomData::<
                        Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                    >,
                )
            }
        }
        impl<Context, App, B> IsProviderFor<NameGetterComponent, Context, (App, B)> for UseFields
        where
            App: HasNameType,
            Context: HasField<
                Char<'n', Char<'a', Char<'m', Char<'e', Nil>>>>,
                Value = App::Name,
            >,
        {
        }
        impl<Context, App, B, __Tag__> NameGetter<Context, App, B> for UseField<__Tag__>
        where
            App: HasNameType,
            Context: HasField<__Tag__, Value = App::Name>,
        {
            fn name(context: &Context, _phantom: PhantomData<(App, B)>) -> &App::Name {
                context.get_field(::core::marker::PhantomData)
            }
        }
        impl<Context, App, B, __Tag__> IsProviderFor<NameGetterComponent, Context, (App, B)>
            for UseField<__Tag__>
        where
            App: HasNameType,
            Context: HasField<__Tag__, Value = App::Name>,
        {
        }
        impl<Context, App, B, __Provider__> NameGetter<Context, App, B> for WithProvider<__Provider__>
        where
            App: HasNameType,
            __Provider__: FieldGetter<Context, NameGetterComponent, Value = App::Name>,
        {
            fn name(context: &Context, _phantom: PhantomData<(App, B)>) -> &App::Name {
                __Provider__::get_field(context, ::core::marker::PhantomData)
            }
        }
        impl<Context, App, B, __Provider__> IsProviderFor<NameGetterComponent, Context, (App, B)>
            for WithProvider<__Provider__>
        where
            App: HasNameType,
            __Provider__: FieldGetter<Context, NameGetterComponent, Value = App::Name>,
        {
        }
    };

    assert_equal_token_stream(&derived, &expected);
}
