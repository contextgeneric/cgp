use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_getter, snapshot_cgp_type, snapshot_delegate_components};

snapshot_cgp_type! {
    #[cgp_type]
    pub trait HasFooType {
        type Foo;
    }

    expand_has_foo_type(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFooType {
            type Foo;
        }
        impl<__Context__> HasFooType for __Context__
        where
            __Context__: FooTypeProvider<__Context__>,
        {
            type Foo = <__Context__ as FooTypeProvider<__Context__>>::Foo;
        }
        pub trait FooTypeProvider<
            __Context__,
        >: IsProviderFor<FooTypeProviderComponent, __Context__, ()> {
            type Foo;
        }
        impl<__Provider__, __Context__> FooTypeProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<FooTypeProviderComponent>
                + IsProviderFor<FooTypeProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                FooTypeProviderComponent,
            >>::Delegate: FooTypeProvider<__Context__>,
        {
            type Foo = <<__Provider__ as DelegateComponent<
                FooTypeProviderComponent,
            >>::Delegate as FooTypeProvider<__Context__>>::Foo;
        }
        pub struct FooTypeProviderComponent;
        impl<__Context__> FooTypeProvider<__Context__> for UseContext
        where
            __Context__: HasFooType,
        {
            type Foo = <__Context__ as HasFooType>::Foo;
        }
        impl<__Context__> IsProviderFor<FooTypeProviderComponent, __Context__, ()> for UseContext
        where
            __Context__: HasFooType,
        {}
        impl<__Context__, __Components__, __Path__> FooTypeProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: FooTypeProvider<__Context__>,
        {
            type Foo = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as FooTypeProvider<__Context__>>::Foo;
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<FooTypeProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<FooTypeProviderComponent, __Context__, ()>
                + FooTypeProvider<__Context__>,
        {}
        impl<Foo, __Context__> FooTypeProvider<__Context__> for UseType<Foo> {
            type Foo = Foo;
        }
        impl<Foo, __Context__> IsProviderFor<FooTypeProviderComponent, __Context__, ()>
        for UseType<Foo> {}
        impl<__Provider__, Foo, __Context__> FooTypeProvider<__Context__>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, FooTypeProviderComponent, Type = Foo>,
        {
            type Foo = Foo;
        }
        impl<
            __Provider__,
            Foo,
            __Context__,
        > IsProviderFor<FooTypeProviderComponent, __Context__, ()> for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, FooTypeProviderComponent, Type = Foo>,
        {}
        ")
    }
}

snapshot_cgp_type! {
    #[cgp_type]
    pub trait HasBarType {
        type Bar;
    }

    expand_has_bar_type(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasBarType {
            type Bar;
        }
        impl<__Context__> HasBarType for __Context__
        where
            __Context__: BarTypeProvider<__Context__>,
        {
            type Bar = <__Context__ as BarTypeProvider<__Context__>>::Bar;
        }
        pub trait BarTypeProvider<
            __Context__,
        >: IsProviderFor<BarTypeProviderComponent, __Context__, ()> {
            type Bar;
        }
        impl<__Provider__, __Context__> BarTypeProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<BarTypeProviderComponent>
                + IsProviderFor<BarTypeProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                BarTypeProviderComponent,
            >>::Delegate: BarTypeProvider<__Context__>,
        {
            type Bar = <<__Provider__ as DelegateComponent<
                BarTypeProviderComponent,
            >>::Delegate as BarTypeProvider<__Context__>>::Bar;
        }
        pub struct BarTypeProviderComponent;
        impl<__Context__> BarTypeProvider<__Context__> for UseContext
        where
            __Context__: HasBarType,
        {
            type Bar = <__Context__ as HasBarType>::Bar;
        }
        impl<__Context__> IsProviderFor<BarTypeProviderComponent, __Context__, ()> for UseContext
        where
            __Context__: HasBarType,
        {}
        impl<__Context__, __Components__, __Path__> BarTypeProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: BarTypeProvider<__Context__>,
        {
            type Bar = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as BarTypeProvider<__Context__>>::Bar;
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<BarTypeProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<BarTypeProviderComponent, __Context__, ()>
                + BarTypeProvider<__Context__>,
        {}
        impl<Bar, __Context__> BarTypeProvider<__Context__> for UseType<Bar> {
            type Bar = Bar;
        }
        impl<Bar, __Context__> IsProviderFor<BarTypeProviderComponent, __Context__, ()>
        for UseType<Bar> {}
        impl<__Provider__, Bar, __Context__> BarTypeProvider<__Context__>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, BarTypeProviderComponent, Type = Bar>,
        {
            type Bar = Bar;
        }
        impl<
            __Provider__,
            Bar,
            __Context__,
        > IsProviderFor<BarTypeProviderComponent, __Context__, ()> for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, BarTypeProviderComponent, Type = Bar>,
        {}
        ")
    }
}

snapshot_cgp_getter! {
    #[cgp_getter]
    pub trait HasFooBar: HasFooType + HasBarType {
        fn foo_bar(foo: &Self::Foo) -> &Self::Bar;
    }

    expand_has_foo_bar(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFooBar: HasFooType + HasBarType {
            fn foo_bar(foo: &Self::Foo) -> &Self::Bar;
        }
        impl<__Context__> HasFooBar for __Context__
        where
            __Context__: HasFooType + HasBarType,
            __Context__: FooBarGetter<__Context__>,
        {
            fn foo_bar(foo: &Self::Foo) -> &Self::Bar {
                __Context__::foo_bar(foo)
            }
        }
        pub trait FooBarGetter<
            __Context__,
        >: IsProviderFor<FooBarGetterComponent, __Context__, ()>
        where
            __Context__: HasFooType + HasBarType,
        {
            fn foo_bar(foo: &__Context__::Foo) -> &__Context__::Bar;
        }
        impl<__Provider__, __Context__> FooBarGetter<__Context__> for __Provider__
        where
            __Context__: HasFooType + HasBarType,
            __Provider__: DelegateComponent<FooBarGetterComponent>
                + IsProviderFor<FooBarGetterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                FooBarGetterComponent,
            >>::Delegate: FooBarGetter<__Context__>,
        {
            fn foo_bar(foo: &__Context__::Foo) -> &__Context__::Bar {
                <__Provider__ as DelegateComponent<
                    FooBarGetterComponent,
                >>::Delegate::foo_bar(foo)
            }
        }
        pub struct FooBarGetterComponent;
        impl<__Context__> FooBarGetter<__Context__> for UseContext
        where
            __Context__: HasFooType + HasBarType,
            __Context__: HasFooBar,
        {
            fn foo_bar(foo: &__Context__::Foo) -> &__Context__::Bar {
                __Context__::foo_bar(foo)
            }
        }
        impl<__Context__> IsProviderFor<FooBarGetterComponent, __Context__, ()> for UseContext
        where
            __Context__: HasFooType + HasBarType,
            __Context__: HasFooBar,
        {}
        impl<__Context__, __Components__, __Path__> FooBarGetter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooType + HasBarType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: FooBarGetter<__Context__>,
        {
            fn foo_bar(foo: &__Context__::Foo) -> &__Context__::Bar {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::foo_bar(foo)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<FooBarGetterComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooType + HasBarType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<FooBarGetterComponent, __Context__, ()>
                + FooBarGetter<__Context__>,
        {}
        impl<__Context__> FooBarGetter<__Context__> for UseFields
        where
            __Context__: HasFooType + HasBarType,
            __Context__::Foo: HasField<
                Symbol<
                    7,
                    Chars<
                        'f',
                        Chars<
                            'o',
                            Chars<'o', Chars<'_', Chars<'b', Chars<'a', Chars<'r', Nil>>>>>,
                        >,
                    >,
                >,
                Value = __Context__::Bar,
            >,
        {
            fn foo_bar(__context__: &__Context__::Foo) -> &__Context__::Bar {
                __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                7,
                                Chars<
                                    'f',
                                    Chars<
                                        'o',
                                        Chars<
                                            'o',
                                            Chars<'_', Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    )
            }
        }
        impl<__Context__> IsProviderFor<FooBarGetterComponent, __Context__, ()> for UseFields
        where
            __Context__: HasFooType + HasBarType,
            __Context__::Foo: HasField<
                Symbol<
                    7,
                    Chars<
                        'f',
                        Chars<
                            'o',
                            Chars<'o', Chars<'_', Chars<'b', Chars<'a', Chars<'r', Nil>>>>>,
                        >,
                    >,
                >,
                Value = __Context__::Bar,
            >,
        {}
        impl<__Context__, __Tag__> FooBarGetter<__Context__> for UseField<__Tag__>
        where
            __Context__: HasFooType + HasBarType,
            __Context__::Foo: HasField<__Tag__, Value = __Context__::Bar>,
        {
            fn foo_bar(__context__: &__Context__::Foo) -> &__Context__::Bar {
                __context__.get_field(::core::marker::PhantomData::<__Tag__>)
            }
        }
        impl<__Context__, __Tag__> IsProviderFor<FooBarGetterComponent, __Context__, ()>
        for UseField<__Tag__>
        where
            __Context__: HasFooType + HasBarType,
            __Context__::Foo: HasField<__Tag__, Value = __Context__::Bar>,
        {}
        impl<__Context__, __Provider__> FooBarGetter<__Context__> for WithProvider<__Provider__>
        where
            __Context__: HasFooType + HasBarType,
            __Provider__: FieldGetter<
                __Context__::Foo,
                FooBarGetterComponent,
                Value = __Context__::Bar,
            >,
        {
            fn foo_bar(__context__: &__Context__::Foo) -> &__Context__::Bar {
                __Provider__::get_field(
                    __context__,
                    ::core::marker::PhantomData::<FooBarGetterComponent>,
                )
            }
        }
        impl<__Context__, __Provider__> IsProviderFor<FooBarGetterComponent, __Context__, ()>
        for WithProvider<__Provider__>
        where
            __Context__: HasFooType + HasBarType,
            __Provider__: FieldGetter<
                __Context__::Foo,
                FooBarGetterComponent,
                Value = __Context__::Bar,
            >,
        {}
        ")
    }
}

pub struct App;

#[derive(HasField)]
pub struct Foo {
    pub bar: u32,
}

snapshot_delegate_components! {
    delegate_components! {
        App {
            FooTypeProviderComponent:
                UseType<Foo>,
            BarTypeProviderComponent:
                UseType<u32>,
            FooBarGetterComponent:
                UseField<Symbol!("bar")>,
        }
    }

    expand_app(output) {
        insta::assert_snapshot!(output, @r#"
        impl DelegateComponent<FooTypeProviderComponent> for App {
            type Delegate = UseType<Foo>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<FooTypeProviderComponent, __Context__, __Params__> for App
        where
            UseType<Foo>: IsProviderFor<FooTypeProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<BarTypeProviderComponent> for App {
            type Delegate = UseType<u32>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<BarTypeProviderComponent, __Context__, __Params__> for App
        where
            UseType<u32>: IsProviderFor<BarTypeProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<FooBarGetterComponent> for App {
            type Delegate = UseField<Symbol!("bar")>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<FooBarGetterComponent, __Context__, __Params__> for App
        where
            UseField<
                Symbol!("bar"),
            >: IsProviderFor<FooBarGetterComponent, __Context__, __Params__>,
        {}
        "#)
    }
}

#[test]
fn test_non_self_getter() {
    let foo = Foo { bar: 42 };

    let bar = <App as HasFooBar>::foo_bar(&foo);
    assert_eq!(bar, &42);
}
