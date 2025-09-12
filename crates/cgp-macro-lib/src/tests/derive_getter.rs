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
            Context: HasCgpProvider,
            Context::CgpProvider: NameGetter<Context>,
        {
            fn name(&self) -> &Self::Name {
                Context::CgpProvider::name(self)
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

        impl<Context> NameGetter<Context> for UseContext
        where
            Context: HasNameType,
            Context: HasName,
        {
            fn name(context: &Context) -> &Context::Name {
                Context::name(context)
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseContext
        where
            Context: HasNameType,
            Context: HasName,
        {}

        impl<Context> NameGetter<Context> for UseFields
        where
            Context: HasNameType,
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                Value = Context::Name,
            >,
        {
            fn name(context: &Context) -> &Context::Name {
                context.get_field(
                    ::core::marker::PhantomData::<
                        ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                    >,
                )
            }
        }

        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseFields
        where
            Context: HasNameType,
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
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
            Context: HasCgpProvider,
            Context::CgpProvider: NameGetter<Context>,
        {
            fn name(&self) -> &str {
                Context::CgpProvider::name(self)
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
        impl<Context> NameGetter<Context> for UseContext
        where
            Context: HasName,
        {
            fn name(context: &Context) -> &str {
                Context::name(context)
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseContext
        where
            Context: HasName,
        {}
        impl<Context> NameGetter<Context> for UseFields
        where
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                Value = String,
            >,
        {
            fn name(context: &Context) -> &str {
                context
                    .get_field(
                        ::core::marker::PhantomData::<
                            ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                        >,
                    )
                    .as_str()
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseFields where
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
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
            Context: HasCgpProvider,
            Context::CgpProvider: NameGetter<Context>,
        {
            fn name(&mut self) -> &mut str {
                Context::CgpProvider::name(self)
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
        impl<Context> NameGetter<Context> for UseContext
        where
            Context: HasName,
        {
            fn name(context: &mut Context) -> &mut str {
                Context::name(context)
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseContext
        where
            Context: HasName,
        {}
        impl<Context> NameGetter<Context> for UseFields
        where
            Context: HasFieldMut<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                Value = String,
            >,
        {
            fn name(context: &mut Context) -> &mut str {
                context
                    .get_field_mut(
                        ::core::marker::PhantomData::<
                            ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                        >,
                    )
                    .as_mut_str()
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseFields where
            Context: HasFieldMut<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
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
            Context: HasCgpProvider,
            Context::CgpProvider: NameGetter<Context>,
        {
            fn name(&self) -> Self::Name {
                Context::CgpProvider::name(self)
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
        impl<Context> NameGetter<Context> for UseContext
        where
            Context: HasNameType<Name: Clone>,
            Context: HasName,
        {
            fn name(context: &Context) -> Context::Name {
                Context::name(context)
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseContext
        where
            Context: HasNameType<Name: Clone>,
            Context: HasName,
        {}
        impl<Context> NameGetter<Context> for UseFields
        where
            Context: HasNameType<Name: Clone>,
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                Value = Context::Name,
            >,
        {
            fn name(context: &Context) -> Context::Name {
                context
                    .get_field(
                        ::core::marker::PhantomData::<
                            ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                        >,
                    )
                    .clone()
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseFields
        where
            Context: HasNameType<Name: Clone>,
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
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
            Context: HasCgpProvider,
            Context::CgpProvider: NameGetter<Context>,
        {
            fn name(&self) -> Option<&Self::Name> {
                Context::CgpProvider::name(self)
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
        impl<Context> NameGetter<Context> for UseContext
        where
            Context: HasNameType,
            Context: HasName,
        {
            fn name(context: &Context) -> Option<&Context::Name> {
                Context::name(context)
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseContext
        where
            Context: HasNameType,
            Context: HasName,
        {}
        impl<Context> NameGetter<Context> for UseFields
        where
            Context: HasNameType,
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                Value = Option<Context::Name>,
            >,
        {
            fn name(context: &Context) -> Option<&Context::Name> {
                context
                    .get_field(
                        ::core::marker::PhantomData::<
                            ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                        >,
                    )
                    .as_ref()
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseFields
        where
            Context: HasNameType,
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
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
            Context: HasCgpProvider,
            Context::CgpProvider: NameGetter<Context>,
        {
            fn name(&mut self) -> Option<&mut Self::Name> {
                Context::CgpProvider::name(self)
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
        impl<Context> NameGetter<Context> for UseContext
        where
            Context: HasNameType,
            Context: HasName,
        {
            fn name(context: &mut Context) -> Option<&mut Context::Name> {
                Context::name(context)
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseContext
        where
            Context: HasNameType,
            Context: HasName,
        {}
        impl<Context> NameGetter<Context> for UseFields
        where
            Context: HasNameType,
            Context: HasFieldMut<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                Value = Option<Context::Name>,
            >,
        {
            fn name(context: &mut Context) -> Option<&mut Context::Name> {
                context
                    .get_field_mut(
                        ::core::marker::PhantomData::<
                            ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                        >,
                    )
                    .as_mut()
            }
        }
        impl<Context> IsProviderFor<NameGetterComponent, Context, ()> for UseFields
        where
            Context: HasNameType,
            Context: HasFieldMut<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
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
            Context: HasCgpProvider,
            Context::CgpProvider: NameGetter<Context, App>,
        {
            fn name(&self) -> &App::Name {
                Context::CgpProvider::name(self)
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
        impl<Context, App> NameGetter<Context, App> for UseContext
        where
            App: HasNameType,
            Context: HasName<App>,
        {
            fn name(context: &Context) -> &App::Name {
                Context::name(context)
            }
        }
        impl<Context, App> IsProviderFor<NameGetterComponent, Context, (App)> for UseContext
        where
            App: HasNameType,
            Context: HasName<App>,
        {}
        impl<Context, App> NameGetter<Context, App> for UseFields
        where
            App: HasNameType,
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                Value = App::Name,
            >,
        {
            fn name(context: &Context) -> &App::Name {
                context.get_field(
                    ::core::marker::PhantomData::<
                        ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                    >,
                )
            }
        }

        impl<Context, App> IsProviderFor<NameGetterComponent, Context, (App)> for UseFields
        where
            App: HasNameType,
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
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
            Context: HasCgpProvider,
            Context::CgpProvider: NameGetter<Context, App>,
        {
            fn name(&self) -> &App::Name {
                Context::CgpProvider::name(self)
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
        impl<Context, App> NameGetter<Context, App> for UseContext
        where
            App: HasNameType,
            Context: HasName<App>,
        {
            fn name(context: &Context) -> &App::Name {
                Context::name(context)
            }
        }
        impl<Context, App> IsProviderFor<NameGetterComponent<App>, Context, (App)> for UseContext
        where
            App: HasNameType,
            Context: HasName<App>,
        {}
        impl<Context, App> NameGetter<Context, App> for UseFields
        where
            App: HasNameType,
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                Value = App::Name,
            >,
        {
            fn name(context: &Context) -> &App::Name {
                context.get_field(
                    ::core::marker::PhantomData::<
                        ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                    >,
                )
            }
        }
        impl<Context, App> IsProviderFor<NameGetterComponent<App>, Context, (App)> for UseFields
        where
            App: HasNameType,
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
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
            Context: HasCgpProvider,
            Context::CgpProvider: NameGetter<Context, App, B>,
        {
            fn name(&self, _phantom: PhantomData<(App, B)>) -> &App::Name {
                Context::CgpProvider::name(self, _phantom)
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
        impl<Context, App, B> NameGetter<Context, App, B> for UseContext
        where
            App: HasNameType,
            Context: HasName<App, B>,
        {
            fn name(context: &Context, _phantom: PhantomData<(App, B)>) -> &App::Name {
                Context::name(context, _phantom)
            }
        }
        impl<Context, App, B> IsProviderFor<NameGetterComponent, Context, (App, B)>
        for UseContext
        where
            App: HasNameType,
            Context: HasName<App, B>,
        {}
        impl<Context, App, B> NameGetter<Context, App, B> for UseFields
        where
            App: HasNameType,
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                Value = App::Name,
            >,
        {
            fn name(context: &Context, _phantom: PhantomData<(App, B)>) -> &App::Name {
                context.get_field(
                    ::core::marker::PhantomData::<
                        ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
                    >,
                )
            }
        }
        impl<Context, App, B> IsProviderFor<NameGetterComponent, Context, (App, B)> for UseFields
        where
            App: HasNameType,
            Context: HasField<
                ι<'n', ι<'a', ι<'m', ι<'e', ε>>>>,
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
